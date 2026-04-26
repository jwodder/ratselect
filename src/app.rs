use crate::{Form, MultiSelector, RadioSelector, Selection, Selector};
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use ratatui::{Terminal, backend::Backend};
use std::collections::BTreeSet;

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
        while self.quitting {
            self.draw(&mut terminal)?;
            self.process_input()?;
        }
        Ok(self.ok.then(|| {
            std::iter::zip(
                self.keys,
                self.lists.into_iter().map(ListData::into_selection),
            )
            .collect()
        }))
    }

    /// Draw the current screen on the terminal
    fn draw<B: Backend>(&self, terminal: &mut Terminal<B>) -> std::io::Result<()>
    where
        std::io::Error: From<B::Error>,
    {
        todo!()
    }

    /// Receive & handle the next input event or lack thereof
    fn process_input(&mut self) -> std::io::Result<()> {
        self.handle_event(read()?);
        Ok(())
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
                KeyCode::Char('w') | KeyCode::PageUp => self.page_up(),
                KeyCode::Char('z') | KeyCode::PageDown => self.page_down(),
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

    fn page_up(&mut self) {
        todo!()
    }

    fn page_down(&mut self) {
        todo!()
    }

    fn goto_top(&mut self) {
        if self.is_empty() {
            self.focus = Focus::OkButton;
        } else {
            self.focus = Focus::Item { list: 0, option: 0 };
            // TODO: Scroll
        }
    }

    fn goto_bottom(&mut self) {
        self.focus = Focus::CancelButton;
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
                    elements.push(Element::Text(title));
                    for (j, opt) in options.into_iter().enumerate() {
                        elements.push(Element::RadioButton {
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
                    elements.push(Element::Text(title));
                    for (j, opt) in options.into_iter().enumerate() {
                        elements.push(Element::Checkbox {
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
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Element {
    Text(String),
    RadioButton {
        list: usize,
        option: usize,
        text: String,
    },
    Checkbox {
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

    fn into_selection(self) -> Selection {
        Selection::Multi(self.checked)
    }
}
