// IMPORTANT: In these tests, `app.render()` needs to be called before the
// first call to `app.handle_event()` so that the `App` fills in the screen
// height in the former call for use in the latter.
use crate::app::*;
use ratatui::{buffer::Buffer, layout::Rect};

mod areas {
    use super::*;

    pub(super) const SCREEN: Rect = Rect::new(0, 0, 80, 24);

    pub(super) const OK: Rect = Rect::new(27, 23, 4, 1);
    //pub(super) const CANCEL: Rect = Rect::new(46, 23, 8, 1);
}

fn mkform() -> Form<char> {
    let mut form = Form::new();
    for (ch, qty) in std::iter::zip("ABCDE".chars(), 16..) {
        form.add(
            ch,
            RadioSelector::new(format!("List {ch}"), (0..qty).map(|i| format!("{ch}{i}"))),
        );
    }
    form
}

#[test]
fn draw_multipage() {
    let mut app = App::from(mkform());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "List A                                                                         ▲",
        "    (X) A0                                                                     █",
        "    ( ) A1                                                                     █",
        "    ( ) A2                                                                     █",
        "    ( ) A3                                                                     █",
        "    ( ) A4                                                                     █",
        "    ( ) A5                                                                     ▒",
        "    ( ) A6                                                                     ▒",
        "    ( ) A7                                                                     ▒",
        "    ( ) A8                                                                     ▒",
        "    ( ) A9                                                                     ▒",
        "    ( ) A10                                                                    ▒",
        "    ( ) A11                                                                    ▒",
        "    ( ) A12                                                                    ▒",
        "    ( ) A13                                                                    ▒",
        "    ( ) A14                                                                    ▒",
        "    ( ) A15                                                                    ▒",
        "                                                                               ▒",
        "List B                                                                         ▒",
        "    (X) B0                                                                     ▒",
        "    ( ) B1                                                                     ▒",
        "    ( ) B2                                                                     ▒",
        "    ( ) B3                                                                     ▒",
        "    ( ) B4                                                                     ▼",
    ]);
    expected.set_style(Rect::new(0, 0, 79, 1), TITLE_STYLE); // "List A"
    expected.set_style(Rect::new(0, 18, 79, 1), TITLE_STYLE); // "List B"
    expected.set_style(Rect::new(4, 1, 7, 1), HIGHLIGHT_STYLE); // "A0"
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn goto_bottom() {
    let mut app = App::from(mkform());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "                                                                               ▲",
        "List E                                                                         ▒",
        "    (X) E0                                                                     ▒",
        "    ( ) E1                                                                     ▒",
        "    ( ) E2                                                                     ▒",
        "    ( ) E3                                                                     ▒",
        "    ( ) E4                                                                     ▒",
        "    ( ) E5                                                                     ▒",
        "    ( ) E6                                                                     ▒",
        "    ( ) E7                                                                     ▒",
        "    ( ) E8                                                                     ▒",
        "    ( ) E9                                                                     ▒",
        "    ( ) E10                                                                    ▒",
        "    ( ) E11                                                                    ▒",
        "    ( ) E12                                                                    ▒",
        "    ( ) E13                                                                    ▒",
        "    ( ) E14                                                                    ▒",
        "    ( ) E15                                                                    ▒",
        "    ( ) E16                                                                    █",
        "    ( ) E17                                                                    █",
        "    ( ) E18                                                                    █",
        "    ( ) E19                                                                    █",
        "                                                                               █",
        "                           <OK>               <Cancel>                         ▼",
    ]);
    expected.set_style(Rect::new(0, 1, 79, 1), TITLE_STYLE); // "List E"
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn scroll_down_one_line() {
    let mut app = App::from(mkform());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);

    for _ in 0..20 {
        app.handle_event(Event::Key(KeyCode::Down.into()));
        assert!(app.get_output().is_none());
    }
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "List A                                                                         ▲",
        "    (X) A0                                                                     █",
        "    ( ) A1                                                                     █",
        "    ( ) A2                                                                     █",
        "    ( ) A3                                                                     █",
        "    ( ) A4                                                                     █",
        "    ( ) A5                                                                     ▒",
        "    ( ) A6                                                                     ▒",
        "    ( ) A7                                                                     ▒",
        "    ( ) A8                                                                     ▒",
        "    ( ) A9                                                                     ▒",
        "    ( ) A10                                                                    ▒",
        "    ( ) A11                                                                    ▒",
        "    ( ) A12                                                                    ▒",
        "    ( ) A13                                                                    ▒",
        "    ( ) A14                                                                    ▒",
        "    ( ) A15                                                                    ▒",
        "                                                                               ▒",
        "List B                                                                         ▒",
        "    (X) B0                                                                     ▒",
        "    ( ) B1                                                                     ▒",
        "    ( ) B2                                                                     ▒",
        "    ( ) B3                                                                     ▒",
        "    ( ) B4                                                                     ▼",
    ]);
    expected.set_style(Rect::new(0, 0, 79, 1), TITLE_STYLE); // "List A"
    expected.set_style(Rect::new(0, 18, 79, 1), TITLE_STYLE); // "List B"
    expected.set_style(Rect::new(4, 23, 7, 1), HIGHLIGHT_STYLE); // "B4"
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "    (X) A0                                                                     ▲",
        "    ( ) A1                                                                     █",
        "    ( ) A2                                                                     █",
        "    ( ) A3                                                                     █",
        "    ( ) A4                                                                     █",
        "    ( ) A5                                                                     █",
        "    ( ) A6                                                                     ▒",
        "    ( ) A7                                                                     ▒",
        "    ( ) A8                                                                     ▒",
        "    ( ) A9                                                                     ▒",
        "    ( ) A10                                                                    ▒",
        "    ( ) A11                                                                    ▒",
        "    ( ) A12                                                                    ▒",
        "    ( ) A13                                                                    ▒",
        "    ( ) A14                                                                    ▒",
        "    ( ) A15                                                                    ▒",
        "                                                                               ▒",
        "List B                                                                         ▒",
        "    (X) B0                                                                     ▒",
        "    ( ) B1                                                                     ▒",
        "    ( ) B2                                                                     ▒",
        "    ( ) B3                                                                     ▒",
        "    ( ) B4                                                                     ▒",
        "    ( ) B5                                                                     ▼",
    ]);
    expected.set_style(Rect::new(0, 17, 79, 1), TITLE_STYLE); // "List B"
    expected.set_style(Rect::new(4, 23, 7, 1), HIGHLIGHT_STYLE); // "B5"
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn scroll_down_across_list() {
    let mut app = App::from(mkform());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);

    for _ in 0..32 {
        app.handle_event(Event::Key(KeyCode::Down.into()));
        assert!(app.get_output().is_none());
    }
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "    ( ) A11                                                                    ▲",
        "    ( ) A12                                                                    ▒",
        "    ( ) A13                                                                    ▒",
        "    ( ) A14                                                                    ▒",
        "    ( ) A15                                                                    █",
        "                                                                               █",
        "List B                                                                         █",
        "    (X) B0                                                                     █",
        "    ( ) B1                                                                     █",
        "    ( ) B2                                                                     ▒",
        "    ( ) B3                                                                     ▒",
        "    ( ) B4                                                                     ▒",
        "    ( ) B5                                                                     ▒",
        "    ( ) B6                                                                     ▒",
        "    ( ) B7                                                                     ▒",
        "    ( ) B8                                                                     ▒",
        "    ( ) B9                                                                     ▒",
        "    ( ) B10                                                                    ▒",
        "    ( ) B11                                                                    ▒",
        "    ( ) B12                                                                    ▒",
        "    ( ) B13                                                                    ▒",
        "    ( ) B14                                                                    ▒",
        "    ( ) B15                                                                    ▒",
        "    ( ) B16                                                                    ▼",
    ]);
    expected.set_style(Rect::new(0, 6, 79, 1), TITLE_STYLE); // "List B"
    expected.set_style(Rect::new(4, 23, 7, 1), HIGHLIGHT_STYLE); // "B16"
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "    ( ) A14                                                                    ▲",
        "    ( ) A15                                                                    ▒",
        "                                                                               ▒",
        "List B                                                                         ▒",
        "    (X) B0                                                                     █",
        "    ( ) B1                                                                     █",
        "    ( ) B2                                                                     █",
        "    ( ) B3                                                                     █",
        "    ( ) B4                                                                     █",
        "    ( ) B5                                                                     ▒",
        "    ( ) B6                                                                     ▒",
        "    ( ) B7                                                                     ▒",
        "    ( ) B8                                                                     ▒",
        "    ( ) B9                                                                     ▒",
        "    ( ) B10                                                                    ▒",
        "    ( ) B11                                                                    ▒",
        "    ( ) B12                                                                    ▒",
        "    ( ) B13                                                                    ▒",
        "    ( ) B14                                                                    ▒",
        "    ( ) B15                                                                    ▒",
        "    ( ) B16                                                                    ▒",
        "                                                                               ▒",
        "List C                                                                         ▒",
        "    (X) C0                                                                     ▼",
    ]);
    expected.set_style(Rect::new(0, 3, 79, 1), TITLE_STYLE); // "List B"
    expected.set_style(Rect::new(0, 22, 79, 1), TITLE_STYLE); // "List C"
    expected.set_style(Rect::new(4, 23, 7, 1), HIGHLIGHT_STYLE); // "C0"
    pretty_assertions::assert_eq!(buffer, expected);
}
