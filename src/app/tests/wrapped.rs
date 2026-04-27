use crate::app::*;
use ratatui::{buffer::Buffer, layout::Rect};

static TITLE: &str =
    "Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod";
static LINE1: &str =
    "quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo";
static LINE2: &str =
    "tincidunt ut laoreet dolore magna aliquam erat volutpat.  Ut wisi enim ad minim veniam,";
static LINE3: &str =
    "consequat.  Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse";

mod areas {
    use super::*;

    pub(super) const SCREEN: Rect = Rect::new(0, 0, 80, 24);

    pub(super) const TITLE: Rect = Rect::new(0, 0, 79, 2);
    pub(super) const LINE1: Rect = Rect::new(4, 2, 74, 2);
}

#[test]
fn draw_multiline() {
    let mut form = Form::new();
    form.add("lorem", RadioSelector::new(TITLE, [LINE1, LINE2, LINE3]));
    let mut app = App::from(form);
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh ",
        "euismod                                                                         ",
        "    (X) quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut        ",
        "        aliquip ex ea commodo                                                   ",
        "    ( ) tincidunt ut laoreet dolore magna aliquam erat volutpat.  Ut wisi enim  ",
        "        ad minim veniam,                                                        ",
        "    ( ) consequat.  Duis autem vel eum iriure dolor in hendrerit in vulputate   ",
        "        velit esse                                                              ",
        "                                                                                ",
        "                           <OK>               <Cancel>                          ",
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
    ]);
    expected.set_style(areas::TITLE, TITLE_STYLE);
    expected.set_style(areas::LINE1, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}
