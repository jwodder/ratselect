use crate::app::*;
use ratatui::{buffer::Buffer, layout::Rect};

mod areas {
    use super::*;

    pub(super) const SCREEN: Rect = Rect::new(0, 0, 80, 24);

    //pub(super) const OK: Rect = Rect::new(27, 23, 4, 1);
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
