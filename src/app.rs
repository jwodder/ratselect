use crate::{Form, Selection};
use crossterm::event::{Event, read};
use ratatui::{Terminal, backend::Backend};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct App<T> {
    keys: Vec<T>,
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
        while !self.quitting() {
            self.draw(&mut terminal)?;
            self.process_input()?;
        }
        todo!()
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
        todo!()
    }

    /// Should the application terminate?
    fn quitting(&self) -> bool {
        todo!()
    }
}

impl<T> From<Form<T>> for App<T> {
    fn from(value: Form<T>) -> App<T> {
        todo!()
    }
}
