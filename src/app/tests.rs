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
    expected.set_style(Rect::new(0, 1, 80, 1), HIGHLIGHT_STYLE); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn ok_flavors() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(Rect::new(26, 16, 8, 1), HIGHLIGHT_STYLE); // "  <OK>  "
    pretty_assertions::assert_eq!(buffer, expected);
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(
        app.get_output(),
        Some(Some(vec![
            ("flavor", Selection::Radio(0)),
            ("toppings", Selection::Multi(BTreeSet::new())),
        ]))
    );
}

#[test]
fn cancel_flavors() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(Rect::new(46, 16, 8, 1), HIGHLIGHT_STYLE); // "<Cancel>"
    pretty_assertions::assert_eq!(buffer, expected);
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(app.get_output(), Some(None));
}

// move around in "flavors" and toggle boxes
// hit "OK" after changing something
// hit "Cancel" after changing something
// activate a radio button, then activate it again
// activate a radio button, then activate another one
// activate a checkbox, then activate it again
// activate a checkbox, then activate another one
