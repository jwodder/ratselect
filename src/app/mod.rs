mod widgets;
use crate::{Form, MultiSelector, RadioSelector, Selection, Selector};
use crossterm::event::{Event, KeyCode, KeyModifiers, poll, read};
use ratatui::{
    Terminal,
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::Widget,
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
    wrap_cache: Option<WrapCache>,
    /// Number of elements of `elements` that are above the top of the screen
    offset: usize,
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
        }
        // TODO: Scroll
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
                } else if let Some(list) = list.checked_sub(1) {
                    self.focus = Focus::Item {
                        list,
                        option: self.lists[list].len() - 1,
                        index: index - 3,
                    };
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
            self.focus = Focus::Item {
                list: 0,
                option: 0,
                index: 1,
            };
            // TODO: Scroll
        }
    }

    fn goto_bottom(&mut self) {
        self.focus = Focus::OkButton;
        // TODO: Scroll
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
        // TODO: Scroll
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
        // TODO: Scroll
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

    fn get_wrapped_elements(&mut self, screen_width: u16) -> (u16, Rc<[Element]>) {
        if let Some(ref wc) = self.wrap_cache
            && wc.screen_width == screen_width
        {
            (wc.option_highlight_width, Rc::clone(&wc.elements))
        } else {
            let max_title_width = usize::from(screen_width.saturating_sub(1));
            let max_option_label_width = usize::from(screen_width.saturating_sub(
                OPTION_INDENT + 4 /* for boxes */ + 1, /* for scrollbar */
            ));
            let mut option_highlight_width = 0;
            let mut elements = Vec::with_capacity(self.elements.len());
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
                            .collect();
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
                            .collect();
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
                            .collect();
                        elements.push(Element::MultiOption {
                            list: *list,
                            option: *option,
                            text: wrapped,
                        });
                    }
                    Element::BlankLine => elements.push(Element::BlankLine),
                    Element::Buttons => elements.push(Element::Buttons),
                }
            }
            let elements = Rc::<[Element]>::from(elements);
            let option_highlight_width = u16::try_from(option_highlight_width)
                .unwrap_or(u16::MAX)
                .saturating_add(4);
            let r = (option_highlight_width, Rc::clone(&elements));
            self.wrap_cache = Some(WrapCache {
                screen_width,
                option_highlight_width,
                elements,
            });
            r
        }
    }
}

impl<T> Widget for &mut App<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (option_highlight_width, elements) = self.get_wrapped_elements(area.width);
        let [mut area, _scrollbar_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
        for (i, elem) in elements.iter().enumerate() {
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
                        Constraint::Length(option_highlight_width),
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
                        Constraint::Length(option_highlight_width),
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
            offset: 0,
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
struct WrapCache {
    screen_width: u16,
    option_highlight_width: u16,
    elements: Rc<[Element]>,
}

fn split_lines(s: &str) -> Vec<String> {
    s.trim_end_matches(['\r', '\n'])
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests;
