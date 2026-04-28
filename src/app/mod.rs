mod widgets;
use crate::{Form, MultiSelector, RadioSelector, Selection, Selector};
use crossterm::event::{Event, KeyCode, KeyModifiers, poll, read};
use ratatui::{
    Terminal,
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::rc::Rc;
use std::time::Duration;
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
    wrap_cache: Option<Rc<WrapInfo>>,
    /// Number of elements of `elements` that are above the top of the screen
    scroll_offset: usize,
    screen_height: Option<u16>,
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
    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()>
    where
        std::io::Error: From<B::Error>,
    {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    /// Receive & handle the next input event or lack thereof
    fn process_input(&mut self) -> std::io::Result<()> {
        let ev = read()?;
        if ev.is_resize() {
            while poll(Duration::from_millis(50))? {
                let ev = read()?;
                if !ev.is_resize() {
                    self.handle_event(ev);
                    break;
                }
            }
            // If we only encountered resize events, return anyway so the loop
            // in `run()` will then redraw the screen.
        } else {
            self.handle_event(ev);
        }
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
            index,
        } = self.focus
        {
            option += 1;
            if option < self.lists[list].len() {
                self.focus = Focus::Item {
                    list,
                    option,
                    index: index + 1,
                };
            } else {
                list += 1;
                if list < self.lists.len() {
                    self.focus = Focus::Item {
                        list,
                        option: 0,
                        index: index + 3,
                    };
                } else {
                    self.focus = Focus::OkButton;
                }
            }
            self.adjscroll();
        }
    }

    fn move_up(&mut self) {
        match self.focus {
            Focus::Item {
                list,
                option,
                index,
            } => {
                if let Some(option) = option.checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option,
                        index: index - 1,
                    };
                    self.adjscroll();
                } else if let Some(list) = list.checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option: self.lists[list].len() - 1,
                        index: index - 3,
                    };
                    self.adjscroll();
                }
                // Else: We're at the top
            }
            Focus::OkButton | Focus::CancelButton => {
                if let Some(list) = self.lists.len().checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option: self.lists[list].len() - 1,
                        index: self.elements.len() - 3,
                    };
                    self.adjscroll();
                }
                // Else: content is empty; can't move up
            }
        }
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
            self.scroll_offset = 0;
            self.focus = Focus::Item {
                list: 0,
                option: 0,
                index: 1,
            };
            self.adjscroll();
        }
    }

    fn goto_bottom(&mut self) {
        self.focus = Focus::OkButton;
        self.adjscroll();
    }

    fn next_block(&mut self) {
        match self.focus {
            Focus::Item {
                mut list,
                option,
                mut index,
                ..
            } => {
                index += self.lists[list].len() - option + 2;
                list += 1;
                if list < self.lists.len() {
                    self.focus = Focus::Item {
                        list,
                        option: 0,
                        index,
                    };
                } else {
                    self.focus = Focus::OkButton;
                }
            }
            Focus::OkButton => self.focus = Focus::CancelButton,
            Focus::CancelButton => {
                if self.is_empty() {
                    self.focus = Focus::OkButton;
                } else {
                    self.focus = Focus::Item {
                        list: 0,
                        option: 0,
                        index: 1,
                    };
                }
            }
        }
        self.adjscroll();
    }

    fn prev_block(&mut self) {
        match self.focus {
            Focus::Item {
                list,
                option,
                index,
            } => {
                if let Some(list) = list.checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option: 0,
                        index: index - option - self.lists[list].len() - 2,
                    };
                } else {
                    self.focus = Focus::CancelButton;
                }
            }
            Focus::OkButton => {
                if let Some(list) = self.lists.len().checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option: 0,
                        index: self.elements.len() - self.lists[list].len() - 2,
                    };
                } else {
                    self.focus = Focus::CancelButton;
                }
            }
            Focus::CancelButton => self.focus = Focus::OkButton,
        }
        self.adjscroll();
    }

    fn adjscroll(&mut self) {
        if let Some(screen_height) = self.screen_height {
            match self.focus {
                Focus::Item { index, .. } => {
                    if index < self.scroll_offset {
                        self.scroll_offset = index;
                    } else {
                        let mut depth = self.elements[self.scroll_offset..=index]
                            .iter()
                            .map(Element::height)
                            .sum::<u16>();
                        while depth > screen_height && self.scroll_offset < index {
                            // Scroll down one full item
                            depth -= self.elements[self.scroll_offset].height();
                            self.scroll_offset += 1;
                        }
                    }
                }
                Focus::OkButton | Focus::CancelButton => {
                    let mut depth = self.elements[self.scroll_offset..]
                        .iter()
                        .map(Element::height)
                        .sum::<u16>();
                    while depth > screen_height && self.scroll_offset < self.elements.len() - 1 {
                        // Scroll down one full item
                        depth -= self.elements[self.scroll_offset].height();
                        self.scroll_offset += 1;
                    }
                }
            }
        }
    }

    fn activate(&mut self) {
        match self.focus {
            Focus::Item { list, option, .. } => self.lists[list].activate_option(option),
            Focus::OkButton => {
                self.ok = true;
                self.quitting = true;
            }
            Focus::CancelButton => self.quitting = true,
        }
    }

    fn get_wrap_info(&mut self, screen_width: u16) -> Rc<WrapInfo> {
        if let Some(ref wc) = self.wrap_cache
            && wc.screen_width == screen_width
        {
            Rc::clone(wc)
        } else {
            let max_title_width = usize::from(screen_width.saturating_sub(1));
            let max_option_label_width = usize::from(screen_width.saturating_sub(
                OPTION_INDENT + 4 /* for boxes */ + 1, /* for scrollbar */
            ));
            let mut option_highlight_width = 0;
            let mut elements = Vec::with_capacity(self.elements.len());
            let mut total_lines = 0;
            for elem in &self.elements {
                match elem {
                    Element::ListTitle(txt) => {
                        let wrapped = txt
                            .iter()
                            .flat_map(|ln| {
                                textwrap::wrap(
                                    ln,
                                    textwrap::Options::new(max_title_width).break_words(true),
                                )
                            })
                            .map(Cow::into_owned)
                            .collect::<Vec<_>>();
                        total_lines += wrapped.len();
                        elements.push(Element::ListTitle(wrapped));
                    }
                    Element::RadioOption { list, option, text } => {
                        let wrapped = text
                            .iter()
                            .flat_map(|ln| {
                                textwrap::wrap(
                                    ln,
                                    textwrap::Options::new(max_option_label_width)
                                        .break_words(true),
                                )
                            })
                            .inspect(|ln| {
                                option_highlight_width = option_highlight_width.max(ln.width());
                            })
                            .map(Cow::into_owned)
                            .collect::<Vec<_>>();
                        total_lines += wrapped.len();
                        elements.push(Element::RadioOption {
                            list: *list,
                            option: *option,
                            text: wrapped,
                        });
                    }
                    Element::MultiOption { list, option, text } => {
                        let wrapped = text
                            .iter()
                            .flat_map(|ln| {
                                textwrap::wrap(
                                    ln,
                                    textwrap::Options::new(max_option_label_width)
                                        .break_words(true),
                                )
                            })
                            .inspect(|ln| {
                                option_highlight_width = option_highlight_width.max(ln.width());
                            })
                            .map(Cow::into_owned)
                            .collect::<Vec<_>>();
                        total_lines += wrapped.len();
                        elements.push(Element::MultiOption {
                            list: *list,
                            option: *option,
                            text: wrapped,
                        });
                    }
                    Element::BlankLine => {
                        total_lines += 1;
                        elements.push(Element::BlankLine);
                    }
                    Element::Buttons => {
                        total_lines += 1;
                        elements.push(Element::Buttons);
                    }
                }
            }
            let option_highlight_width = u16::try_from(option_highlight_width)
                .unwrap_or(u16::MAX)
                .saturating_add(4);
            let wi = Rc::new(WrapInfo {
                screen_width,
                option_highlight_width,
                elements,
                total_lines,
            });
            self.wrap_cache = Some(Rc::clone(&wi));
            wi
        }
    }
}

impl<T> Widget for &mut App<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wi = self.get_wrap_info(area.width);
        let [mut area, scrollbar_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
        self.screen_height = Some(area.height);
        let max_scroll = wi.total_lines.saturating_sub(usize::from(area.height) - 1);
        let mut scroll_position = 0;
        for (i, elem) in wi.elements.iter().enumerate() {
            if i < self.scroll_offset {
                scroll_position += usize::from(elem.height());
                continue;
            }
            if area.is_empty() {
                break;
            }
            let [rows, a2] =
                Layout::vertical([Constraint::Length(elem.height()), Constraint::Fill(1)])
                    .areas(area);
            area = a2;
            match elem {
                Element::ListTitle(txt) => {
                    widgets::ListTitle(txt).render(rows, buf);
                }
                Element::RadioOption { list, option, text } => {
                    let list = *list;
                    let option = *option;
                    let wdgt = widgets::RadioOption {
                        label: text,
                        checked: self.lists[list].is_checked(option),
                        focused: self.focus
                            == (Focus::Item {
                                list,
                                option,
                                index: i,
                            }),
                    };
                    let [_, line_area, _] = Layout::horizontal([
                        Constraint::Length(OPTION_INDENT),
                        Constraint::Length(wi.option_highlight_width),
                        Constraint::Fill(1),
                    ])
                    .areas(rows);
                    wdgt.render(line_area, buf);
                }
                Element::MultiOption { list, option, text } => {
                    let list = *list;
                    let option = *option;
                    let wdgt = widgets::MultiOption {
                        label: text,
                        checked: self.lists[list].is_checked(option),
                        focused: self.focus
                            == (Focus::Item {
                                list,
                                option,
                                index: i,
                            }),
                    };
                    let [_, line_area, _] = Layout::horizontal([
                        Constraint::Length(OPTION_INDENT),
                        Constraint::Length(wi.option_highlight_width),
                        Constraint::Fill(1),
                    ])
                    .areas(rows);
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
                    .areas(rows);
                    ok_button.render(ok_area, buf);
                    cancel_button.render(cancel_area, buf);
                }
            }
        }
        if wi.total_lines > usize::from(area.height) {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .track_symbol(Some(ratatui::symbols::shade::MEDIUM));
            let mut scroll_state = ScrollbarState::new(max_scroll).position(scroll_position);
            scrollbar.render(scrollbar_area, buf, &mut scroll_state);
        }
    }
}

impl<T> From<Form<T>> for App<T> {
    fn from(form: Form<T>) -> App<T> {
        let capacity = form.selectors.len();
        let mut keys = Vec::with_capacity(capacity);
        let mut lists = Vec::with_capacity(capacity);
        let mut elements = Vec::with_capacity(capacity.saturating_mul(3));
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
                    elements.push(Element::ListTitle(split_lines(&title)));

                    for (j, opt) in options.into_iter().enumerate() {
                        elements.push(Element::RadioOption {
                            list: i,
                            option: j,
                            text: split_lines(&opt),
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
                    elements.push(Element::ListTitle(split_lines(&title)));
                    for (j, opt) in options.into_iter().enumerate() {
                        elements.push(Element::MultiOption {
                            list: i,
                            option: j,
                            text: split_lines(&opt),
                        });
                    }
                }
            }
            elements.push(Element::BlankLine);
        }
        let focus = if keys.is_empty() {
            Focus::OkButton
        } else {
            Focus::Item {
                list: 0,
                option: 0,
                index: 1,
            }
        };
        elements.push(Element::Buttons);
        App {
            keys,
            lists,
            elements,
            focus,
            quitting: false,
            ok: false,
            wrap_cache: None,
            scroll_offset: 0,
            screen_height: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Element {
    ListTitle(Vec<String>),
    RadioOption {
        list: usize,
        option: usize,
        text: Vec<String>,
    },
    MultiOption {
        list: usize,
        option: usize,
        text: Vec<String>,
    },
    BlankLine,
    Buttons,
}

impl Element {
    fn height(&self) -> u16 {
        let len = match self {
            Element::ListTitle(txt) => txt.len(),
            Element::RadioOption { text, .. } => text.len(),
            Element::MultiOption { text, .. } => text.len(),
            Element::BlankLine => 1,
            Element::Buttons => 1,
        };
        u16::try_from(len).unwrap_or(u16::MAX)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Focus {
    Item {
        /// Index within `App::lists` of the selection list within which the
        /// focus is currently located
        list: usize,

        /// Index of the option of the focused selection list that has the
        /// focus
        option: usize,

        /// Index within `App::elements` of the `RadioOption` or `MultiOption`
        /// that has the focus
        index: usize,
    },
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct WrapInfo {
    screen_width: u16,
    option_highlight_width: u16,
    elements: Vec<Element>,
    total_lines: usize,
}

fn split_lines(s: &str) -> Vec<String> {
    s.trim_end_matches(['\r', '\n'])
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests;
