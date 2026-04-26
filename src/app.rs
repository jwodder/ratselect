use crate::{Form, RadioSelector, Selection, Selector};
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use ratatui::{Terminal, backend::Backend};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct App<T> {
    selections: Vec<(T, Selection)>,

    elements: Vec<Element>,

    ok: bool,

    /// Should the application terminate?
    quitting: bool,
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
        Ok(self.ok.then_some(self.selections))
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

    fn move_left(&mut self) {
        todo!()
    }

    fn move_down(&mut self) {
        todo!()
    }

    fn move_up(&mut self) {
        todo!()
    }

    fn move_right(&mut self) {
        todo!()
    }

    fn page_up(&mut self) {
        todo!()
    }

    fn page_down(&mut self) {
        todo!()
    }

    fn goto_top(&mut self) {
        todo!()
    }

    fn goto_bottom(&mut self) {
        todo!()
    }

    fn next_block(&mut self) {
        todo!()
    }

    fn prev_block(&mut self) {
        todo!()
    }

    fn activate(&mut self) {
        todo!()
    }
}

impl<T> From<Form<T>> for App<T> {
    fn from(form: Form<T>) -> App<T> {
        let mut selections = Vec::with_capacity(form.selectors.len());
        let mut elements = Vec::with_capacity(form.selectors.len().saturating_mul(3));
        for (key, s) in form.selectors {
            if !s.is_empty() {
                let i = selections.len();
                selections.push((key, s.default_selection()));
                if !elements.is_empty() {
                    elements.push(Element::BlankLine);
                }
                elements.push(Element::Text(s.title().to_owned()));
                match s {
                    Selector::Radio(RadioSelector {
                        options, default, ..
                    }) => {
                        elements.push(Element::RadioGroup {
                            list_index: i,
                            options,
                            checked: default,
                        });
                    }
                    Selector::Multi(ms) => {
                        elements.push(Element::MultiGroup {
                            list_index: i,
                            options: ms.into_checked_options().collect(),
                        });
                    }
                }
            }
        }
        elements.push(Element::Buttons);
        App {
            selections,
            elements,
            ok: false,
            quitting: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Element {
    Text(String),
    RadioGroup {
        list_index: usize,
        options: Vec<String>,
        checked: usize,
    },
    MultiGroup {
        list_index: usize,
        options: Vec<(bool, String)>,
    },
    BlankLine,
    Buttons,
}
