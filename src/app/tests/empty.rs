use crate::app::*;
use ratatui::{buffer::Buffer, layout::Rect};

const LINES: [&str; 24] = [
    "                            <OK>              <Cancel>                          ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
];

mod areas {
    use super::*;

    pub(super) const SCREEN: Rect = Rect::new(0, 0, 80, 24);

    pub(super) const OK: Rect = Rect::new(28, 0, 4, 1);
    pub(super) const CANCEL: Rect = Rect::new(46, 0, 8, 1);
}

#[test]
fn draw_empty() {
    let app = App::<()>::from(Form::new());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn ok_empty() {
    let mut app = App::<()>::from(Form::new());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(app.get_output(), Some(Some(Vec::new())));
}

#[test]
fn tab_around() {
    let mut app = App::<()>::from(Form::new());

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn shift_tab_around() {
    let mut app = App::<()>::from(Form::new());

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn up_and_down_from_ok() {
    let mut app = App::<()>::from(Form::new());

    app.handle_event(Event::Key(KeyCode::Up.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn up_and_down_from_cancel() {
    let mut app = App::<()>::from(Form::new());

    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Up.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn home() {
    let mut app = App::<()>::from(Form::new());

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn end() {
    let mut app = App::<()>::from(Form::new());

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines(LINES);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}
