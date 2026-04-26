use super::*;
use ratatui::{buffer::Buffer, layout::Rect};

const FLAVORS: [&str; 7] = [
    "Vanilla",
    "Chocolate",
    "Strawberry",
    "Cinnamon",
    "Butterscotch",
    "Peanut Butter Fudge",
    "Chili",
];

const TOPPINGS: [&str; 5] = ["Whipped Cream", "Hot Fudge", "Nuts", "Cherry", "Banana"];

#[test]
fn draw_flavors() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let app = App::from(form);
    let area = Rect::new(0, 0, 80, 24);
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    (X) Vanilla                                                                 ",
        "    ( ) Chocolate                                                               ",
        "    ( ) Strawberry                                                              ",
        "    ( ) Cinnamon                                                                ",
        "    ( ) Butterscotch                                                            ",
        "    ( ) Peanut Butter Fudge                                                     ",
        "    ( ) Chili                                                                   ",
        "                                                                                ",
        "Toppings:                                                                       ",
        "    [ ] Whipped Cream                                                           ",
        "    [ ] Hot Fudge                                                               ",
        "    [ ] Nuts                                                                    ",
        "    [ ] Cherry                                                                  ",
        "    [ ] Banana                                                                  ",
        "                                                                                ",
        "                            <OK>              <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(Rect::new(0, 0, 80, 1), TITLE_STYLE); // "Flavors:"
    expected.set_style(Rect::new(0, 9, 80, 1), TITLE_STYLE); // "Toppings:"
    expected.set_style(Rect::new(0, 1, 80, 1), HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}
