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

const OPTION_HIGHLIGHT_WIDTH: u16 = 23;

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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
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

#[test]
fn double_click_radio() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Chocolate" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    (X) Chocolate                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Chocolate" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(
        app.get_output(),
        Some(Some(vec![
            ("flavor", Selection::Radio(1)),
            ("toppings", Selection::Multi(BTreeSet::new())),
        ]))
    );
}

#[test]
fn double_click_checkbox() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Whipped Cream                                                           ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
fn change_and_cancel() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Chocolate" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    (X) Chocolate                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Chocolate" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(app.get_output(), Some(None));
}

#[test]
fn click_two_radios() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Chocolate" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    (X) Chocolate                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Chocolate" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    (X) Chocolate                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 4, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Cinnamon" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    ( ) Chocolate                                                               ",
        "    ( ) Strawberry                                                              ",
        "    (X) Cinnamon                                                                ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 4, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Cinnamon" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(
        app.get_output(),
        Some(Some(vec![
            ("flavor", Selection::Radio(3)),
            ("toppings", Selection::Multi(BTreeSet::new())),
        ]))
    );
}

#[test]
fn check_two_boxes() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 11, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Hot Fudge" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Hot Fudge                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 11, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Hot Fudge" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Hot Fudge                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 14, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Banana" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Hot Fudge                                                               ",
        "    [ ] Nuts                                                                    ",
        "    [ ] Cherry                                                                  ",
        "    [X] Banana                                                                  ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 14, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Banana" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(
        app.get_output(),
        Some(Some(vec![
            ("flavor", Selection::Radio(0)),
            ("toppings", Selection::Multi(BTreeSet::from([1, 4]))),
        ]))
    );
}

#[test]
fn check_two_boxes_then_uncheck_first() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Whipped Cream                                                           ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Whipped Cream                                                           ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 11, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Hot Fudge" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Whipped Cream                                                           ",
        "    [X] Hot Fudge                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 11, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Hot Fudge" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Up.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Whipped Cream                                                           ",
        "    [X] Hot Fudge                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
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
        "    [X] Hot Fudge                                                               ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(
        app.get_output(),
        Some(Some(vec![
            ("flavor", Selection::Radio(0)),
            ("toppings", Selection::Multi(BTreeSet::from([1]))),
        ]))
    );
}

#[test]
fn tab_around() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn shift_tab_around() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_tab_shift_tab() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 4, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Cinnamon" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_shift_tab_tab() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 4, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Cinnamon" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_all_the_way() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    for _ in 0..12 {
        app.handle_event(Event::Key(KeyCode::Down.into()));
        assert!(app.get_output().is_none());
    }
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

    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::Left.into()));
    assert!(app.get_output().is_none());
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
}

#[test]
fn goto_bottom() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_then_goto_top() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 3, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Strawberry" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_tab_then_goto_top() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 3, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Strawberry" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Whipped Cream" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn goto_top_from_ok() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 3, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Strawberry" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn goto_top_from_cancel() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());
    let area = Rect::new(0, 0, 80, 24);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 3, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Strawberry" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
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

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn ok_custom_defaults() {
    let mut form = Form::new();
    form.add(
        "flavor",
        RadioSelector::new("Flavors:", FLAVORS).with_default(5),
    );
    form.add(
        "toppings",
        MultiSelector::new("Toppings:", TOPPINGS).with_defaults([1, 2]),
    );
    let mut app = App::from(form);
    let area = Rect::new(0, 0, 80, 24);

    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    ( ) Chocolate                                                               ",
        "    ( ) Strawberry                                                              ",
        "    ( ) Cinnamon                                                                ",
        "    ( ) Butterscotch                                                            ",
        "    (X) Peanut Butter Fudge                                                     ",
        "    ( ) Chili                                                                   ",
        "                                                                                ",
        "Toppings:                                                                       ",
        "    [ ] Whipped Cream                                                           ",
        "    [X] Hot Fudge                                                               ",
        "    [X] Nuts                                                                    ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(
        app.get_output(),
        Some(Some(vec![
            ("flavor", Selection::Radio(5)),
            ("toppings", Selection::Multi(BTreeSet::from([1, 2]))),
        ]))
    );
}

#[test]
fn change_custom_defaults() {
    let mut form = Form::new();
    form.add(
        "flavor",
        RadioSelector::new("Flavors:", FLAVORS).with_default(5),
    );
    form.add(
        "toppings",
        MultiSelector::new("Toppings:", TOPPINGS).with_defaults([1, 2]),
    );
    let mut app = App::from(form);
    let area = Rect::new(0, 0, 80, 24);

    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    ( ) Chocolate                                                               ",
        "    ( ) Strawberry                                                              ",
        "    ( ) Cinnamon                                                                ",
        "    ( ) Butterscotch                                                            ",
        "    (X) Peanut Butter Fudge                                                     ",
        "    ( ) Chili                                                                   ",
        "                                                                                ",
        "Toppings:                                                                       ",
        "    [ ] Whipped Cream                                                           ",
        "    [X] Hot Fudge                                                               ",
        "    [X] Nuts                                                                    ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Vanilla" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    (X) Chocolate                                                               ",
        "    ( ) Strawberry                                                              ",
        "    ( ) Cinnamon                                                                ",
        "    ( ) Butterscotch                                                            ",
        "    ( ) Peanut Butter Fudge                                                     ",
        "    ( ) Chili                                                                   ",
        "                                                                                ",
        "Toppings:                                                                       ",
        "    [ ] Whipped Cream                                                           ",
        "    [X] Hot Fudge                                                               ",
        "    [X] Nuts                                                                    ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Chocolate" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    let mut buffer = Buffer::empty(area);
    app.render(area, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Flavors:                                                                        ",
        "    ( ) Vanilla                                                                 ",
        "    (X) Chocolate                                                               ",
        "    ( ) Strawberry                                                              ",
        "    ( ) Cinnamon                                                                ",
        "    ( ) Butterscotch                                                            ",
        "    ( ) Peanut Butter Fudge                                                     ",
        "    ( ) Chili                                                                   ",
        "                                                                                ",
        "Toppings:                                                                       ",
        "    [X] Whipped Cream                                                           ",
        "    [ ] Hot Fudge                                                               ",
        "    [X] Nuts                                                                    ",
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
    expected.set_style(
        Rect::new(OPTION_INDENT, 11, OPTION_HIGHLIGHT_WIDTH, 1),
        HIGHLIGHT_STYLE,
    ); // "Hot Fudge" line
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert_eq!(
        app.get_output(),
        Some(Some(vec![
            ("flavor", Selection::Radio(1)),
            ("toppings", Selection::Multi(BTreeSet::from([0, 2]))),
        ]))
    );
}
