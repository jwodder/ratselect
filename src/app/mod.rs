mod widgets;
use crate::{Form, MultiSelector, RadioSelector, Selection, Selector};
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use ratatui::{
    Terminal,
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::Widget,
};
use std::collections::BTreeSet;
use unicode_width::UnicodeWidthStr;

const OPTION_INDENT: u16 = 4;

const HIGHLIGHT_STYLE: Style = Style::new().reversed();

const TITLE_STYLE: Style = Style::new().bold();

const BUTTON_BOX_WIDTH: u16 = 8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct App<T> {
    keys: Vec<T>,
    lists: Vec<ListData>,
    elements: Vec<Element>,
    focus: Focus,
    /// Should the application terminate?
    quitting: bool,
    /// Did the user end the form by pressing "OK"?
    ok: bool,
    option_highlight_width: u16,
}

impl<T> App<T> {
    /// Run the application on the given terminal
    pub(crate) fn run<B: Backend>(
        mut self,
        mut terminal: Terminal<B>,
    ) -> std::io::Result<Option<Vec<(T, Selection)>>>
    where
        std::io::Error: From<B::Error>,
    {
        loop {
            if let Some(output) = self.get_output() {
                return Ok(output);
            }
            self.draw(&mut terminal)?;
            self.process_input()?;
        }
    }

    /// Draw the current screen on the terminal
    fn draw<B: Backend>(&self, terminal: &mut Terminal<B>) -> std::io::Result<()>
    where
        std::io::Error: From<B::Error>,
    {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    /// Receive & handle the next input event or lack thereof
    fn process_input(&mut self) -> std::io::Result<()> {
        self.handle_event(read()?);
        Ok(())
    }

    /// If the user has either completed or cancelled the application, return
    /// `Some(output)`, where `output` is the value to return from `run()`.
    ///
    /// Once this method returns `Some`, no further methods of `App` should be
    /// called.
    fn get_output(&mut self) -> Option<Option<Vec<(T, Selection)>>> {
        match (self.quitting, self.ok) {
            (true, true) => Some(Some(
                std::iter::zip(
                    self.keys.drain(..),
                    self.lists.drain(..).map(ListData::into_selection),
                )
                .collect(),
            )),
            (true, false) => Some(None),
            (false, _) => None,
        }
    }

    /// Handle the given input event.
    fn handle_event(&mut self, event: Event) {
        let Some(ev) = event.as_key_press_event() else {
            return;
        };
        if (ev.modifiers, ev.code) == (KeyModifiers::CONTROL, KeyCode::Char('c')) {
            self.quitting = true;
        } else if matches!(ev.modifiers, KeyModifiers::NONE | KeyModifiers::SHIFT) {
            match ev.code {
                KeyCode::Char('q' | 'Q') | KeyCode::Esc => self.quitting = true,
                KeyCode::Char('h') | KeyCode::Left => self.move_left(),
                KeyCode::Char('j') | KeyCode::Down => self.move_down(),
                KeyCode::Char('k') | KeyCode::Up => self.move_up(),
                KeyCode::Char('l') | KeyCode::Right => self.move_right(),
                //TODO: KeyCode::Char('w') | KeyCode::PageUp => self.page_up(),
                //TODO: KeyCode::Char('z') | KeyCode::PageDown => self.page_down(),
                KeyCode::Char('g') | KeyCode::Home => self.goto_top(),
                KeyCode::Char('G') | KeyCode::End => self.goto_bottom(),
                KeyCode::Tab => self.next_block(),
                KeyCode::BackTab => self.prev_block(),
                KeyCode::Char(' ') | KeyCode::Enter => self.activate(),
                _ => (),
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    fn move_left(&mut self) {
        if self.focus == Focus::CancelButton {
            self.focus = Focus::OkButton;
        }
    }

    fn move_down(&mut self) {
        if let Focus::Item {
            mut list,
            mut option,
        } = self.focus
        {
            option += 1;
            if option < self.lists[list].len() {
                self.focus = Focus::Item { list, option };
            } else {
                list += 1;
                if list < self.lists.len() {
                    self.focus = Focus::Item { list, option: 0 };
                } else {
                    self.focus = Focus::OkButton;
                }
            }
        }
        // TODO: Scroll
    }

    fn move_up(&mut self) {
        match self.focus {
            Focus::Item { list, option } => {
                if let Some(option) = option.checked_sub(1) {
                    self.focus = Focus::Item { list, option };
                } else if let Some(list) = list.checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option: self.lists[list].len() - 1,
                    };
                }
                // Else: We're at the top
            }
            Focus::OkButton | Focus::CancelButton => {
                if let Some(list) = self.lists.len().checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option: self.lists[list].len() - 1,
                    };
                }
                // Else: content is empty; can't move up
            }
        }
        // TODO: Scroll
    }

    fn move_right(&mut self) {
        if self.focus == Focus::OkButton {
            self.focus = Focus::CancelButton;
        }
    }

    /*
    fn page_up(&mut self) {
        todo!()
    }

    fn page_down(&mut self) {
        todo!()
    }
    */

    fn goto_top(&mut self) {
        if self.is_empty() {
            self.focus = Focus::OkButton;
        } else {
            self.focus = Focus::Item { list: 0, option: 0 };
            // TODO: Scroll
        }
    }

    fn goto_bottom(&mut self) {
        self.focus = Focus::OkButton;
        // TODO: Scroll
    }

    fn next_block(&mut self) {
        match self.focus {
            Focus::Item { mut list, .. } => {
                list += 1;
                if list < self.lists.len() {
                    self.focus = Focus::Item { list, option: 0 };
                } else {
                    self.focus = Focus::OkButton;
                }
            }
            Focus::OkButton => self.focus = Focus::CancelButton,
            Focus::CancelButton => {
                if self.is_empty() {
                    self.focus = Focus::OkButton;
                } else {
                    self.focus = Focus::Item { list: 0, option: 0 };
                }
            }
        }
        // TODO: Scroll
    }

    fn prev_block(&mut self) {
        match self.focus {
            Focus::Item { list, .. } => {
                if let Some(list) = list.checked_sub(1) {
                    self.focus = Focus::Item { list, option: 0 };
                } else {
                    self.focus = Focus::CancelButton;
                }
            }
            Focus::OkButton => {
                if let Some(list) = self.lists.len().checked_sub(1) {
                    self.focus = Focus::Item { list, option: 0 };
                } else {
                    self.focus = Focus::CancelButton;
                }
            }
            Focus::CancelButton => self.focus = Focus::OkButton,
        }
        // TODO: Scroll
    }

    fn activate(&mut self) {
        match self.focus {
            Focus::Item { list, option } => self.lists[list].activate_option(option),
            Focus::OkButton => {
                self.ok = true;
                self.quitting = true;
            }
            Focus::CancelButton => self.quitting = true,
        }
    }
}

impl<T> Widget for &App<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (row, elem) in std::iter::zip(area.rows(), &self.elements) {
            match elem {
                Element::ListTitle(txt) => {
                    widgets::ListTitle(txt).render(row, buf);
                }
                Element::RadioOption { list, option, text } => {
                    let list = *list;
                    let option = *option;
                    let wdgt = widgets::RadioOption {
                        label: text,
                        checked: self.lists[list].is_checked(option),
                        focused: self.focus == (Focus::Item { list, option }),
                    };
                    let [_, line_area, _] = Layout::horizontal([
                        Constraint::Length(OPTION_INDENT),
                        Constraint::Length(self.option_highlight_width),
                        Constraint::Fill(1),
                    ])
                    .areas(row);
                    wdgt.render(line_area, buf);
                }
                Element::MultiOption { list, option, text } => {
                    let list = *list;
                    let option = *option;
                    let wdgt = widgets::MultiOption {
                        label: text,
                        checked: self.lists[list].is_checked(option),
                        focused: self.focus == (Focus::Item { list, option }),
                    };
                    let [_, line_area, _] = Layout::horizontal([
                        Constraint::Length(OPTION_INDENT),
                        Constraint::Length(self.option_highlight_width),
                        Constraint::Fill(1),
                    ])
                    .areas(row);
                    wdgt.render(line_area, buf);
                }
                Element::BlankLine => (),
                Element::Buttons => {
                    let ok_button = widgets::OkButton(self.focus == Focus::OkButton);
                    let cancel_button = widgets::CancelButton(self.focus == Focus::CancelButton);
                    let [_, ok_area, _, cancel_area, _] = Layout::horizontal([
                        Constraint::Fill(2),
                        Constraint::Length(BUTTON_BOX_WIDTH),
                        Constraint::Fill(1),
                        Constraint::Length(BUTTON_BOX_WIDTH),
                        Constraint::Fill(2),
                    ])
                    .areas(row);
                    ok_button.render(ok_area, buf);
                    cancel_button.render(cancel_area, buf);
                }
            }
        }
    }
}

impl<T> From<Form<T>> for App<T> {
    fn from(form: Form<T>) -> App<T> {
        let capacity = form.selectors.len();
        let mut keys = Vec::with_capacity(capacity);
        let mut lists = Vec::with_capacity(capacity);
        let mut elements = Vec::with_capacity(capacity.saturating_mul(3));
        let mut option_highlight_width = 0;
        for (i, (key, s)) in form
            .selectors
            .into_iter()
            .filter(|(_, s)| !s.is_empty())
            .enumerate()
        {
            keys.push(key);
            match s {
                Selector::Radio(RadioSelector {
                    title,
                    options,
                    default,
                }) => {
                    lists.push(ListData::Radio(RadioData {
                        len: options.len(),
                        checked: default,
                    }));
                    elements.push(Element::ListTitle(title));
                    for (j, opt) in options.into_iter().enumerate() {
                        option_highlight_width = option_highlight_width.max(opt.width());
                        elements.push(Element::RadioOption {
                            list: i,
                            option: j,
                            text: opt,
                        });
                    }
                }
                Selector::Multi(MultiSelector {
                    title,
                    options,
                    defaults,
                }) => {
                    lists.push(ListData::Multi(MultiData {
                        len: options.len(),
                        checked: defaults,
                    }));
                    elements.push(Element::ListTitle(title));
                    for (j, opt) in options.into_iter().enumerate() {
                        option_highlight_width = option_highlight_width.max(opt.width());
                        elements.push(Element::MultiOption {
                            list: i,
                            option: j,
                            text: opt,
                        });
                    }
                }
            }
            elements.push(Element::BlankLine);
        }
        let focus = if keys.is_empty() {
            Focus::OkButton
        } else {
            Focus::Item { list: 0, option: 0 }
        };
        elements.push(Element::Buttons);
        App {
            keys,
            lists,
            elements,
            focus,
            quitting: false,
            ok: false,
            option_highlight_width: u16::try_from(option_highlight_width)
                .unwrap_or(u16::MAX)
                .saturating_add(4),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Element {
    ListTitle(String),
    RadioOption {
        list: usize,
        option: usize,
        text: String,
    },
    MultiOption {
        list: usize,
        option: usize,
        text: String,
    },
    BlankLine,
    Buttons,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Focus {
    Item { list: usize, option: usize },
    OkButton,
    CancelButton,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ListData {
    Radio(RadioData),
    Multi(MultiData),
}

impl ListData {
    fn len(&self) -> usize {
        match self {
            ListData::Radio(d) => d.len,
            ListData::Multi(d) => d.len,
        }
    }

    fn activate_option(&mut self, option: usize) {
        match self {
            ListData::Radio(d) => d.activate_option(option),
            ListData::Multi(d) => d.activate_option(option),
        }
    }

    fn is_checked(&self, option: usize) -> bool {
        match self {
            ListData::Radio(d) => d.is_checked(option),
            ListData::Multi(d) => d.is_checked(option),
        }
    }

    fn into_selection(self) -> Selection {
        match self {
            ListData::Radio(d) => d.into_selection(),
            ListData::Multi(d) => d.into_selection(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RadioData {
    len: usize,
    checked: usize,
}

impl RadioData {
    fn activate_option(&mut self, option: usize) {
        self.checked = option;
    }

    fn is_checked(&self, option: usize) -> bool {
        self.checked == option
    }

    fn into_selection(self) -> Selection {
        Selection::Radio(self.checked)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MultiData {
    len: usize,
    checked: BTreeSet<usize>,
}

impl MultiData {
    fn activate_option(&mut self, option: usize) {
        if self.checked.contains(&option) {
            self.checked.remove(&option);
        } else {
            self.checked.insert(option);
        }
    }

    fn is_checked(&self, option: usize) -> bool {
        self.checked.contains(&option)
    }

    fn into_selection(self) -> Selection {
        Selection::Multi(self.checked)
    }
}

#[cfg(test)]
mod tests;
