use crate::app::*;
use ratatui::{buffer::Buffer, layout::Rect};

static VERSE1: &str = concat!(
    "'Twas brillig, and the slithy toves\n",
    "    Did gyre and gimble in the wabe;\n",
    "All mimsy were the borogoves,\n",
    "    And the mome raths outgrabe.\n",
);

static VERSE2: &str = concat!(
    "\"Beware the Jabberwock, my son!\n",
    "    The jaws that bite, the claws that catch!\n",
    "Beware the Jubjub bird, and shun\n",
    "    The frumious Bandersnatch!\"",
);

static VERSE3: &str = concat!(
    "He took his vorpal sword in hand:\n",
    "    Long time the manxome foe he sought--\n",
    "So rested he by the Tumtum tree,\n",
    "    And stood awhile in thought.\n",
);

static VERSE4: &str = concat!(
    "And as in uffish thought he stood,\n",
    "    The Jabberwock, with eyes of flame,\n",
    "Came whiffling through the tulgey wood,\n",
    "    And burbled as it came!\n",
);

mod areas {
    use super::*;

    pub(super) const SCREEN: Rect = Rect::new(0, 0, 80, 24);

    pub(super) const VERSE1: Rect = Rect::new(0, 0, 79, 4);
    pub(super) const VERSE2: Rect = Rect::new(4, 4, 49, 4);
}

#[test]
fn draw_multiline() {
    let mut form = Form::new();
    form.add(
        "jabberwocky",
        RadioSelector::new(VERSE1, [VERSE2, VERSE3, VERSE4]),
    );
    let mut app = App::from(form);
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "'Twas brillig, and the slithy toves                                             ",
        "    Did gyre and gimble in the wabe;                                            ",
        "All mimsy were the borogoves,                                                   ",
        "    And the mome raths outgrabe.                                                ",
        "    (X) \"Beware the Jabberwock, my son!                                         ",
        "            The jaws that bite, the claws that catch!                           ",
        "        Beware the Jubjub bird, and shun                                        ",
        "            The frumious Bandersnatch!\"                                         ",
        "    ( ) He took his vorpal sword in hand:                                       ",
        "            Long time the manxome foe he sought--                               ",
        "        So rested he by the Tumtum tree,                                        ",
        "            And stood awhile in thought.                                        ",
        "    ( ) And as in uffish thought he stood,                                      ",
        "            The Jabberwock, with eyes of flame,                                 ",
        "        Came whiffling through the tulgey wood,                                 ",
        "            And burbled as it came!                                             ",
        "                                                                                ",
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::VERSE1, TITLE_STYLE);
    expected.set_style(areas::VERSE2, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}
