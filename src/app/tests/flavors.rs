use crate::app::*;
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

mod areas {
    use super::*;

    const OPTION_HIGHLIGHT_WIDTH: u16 = 23;

    pub(super) const SCREEN: Rect = Rect::new(0, 0, 80, 24);

    pub(super) const FLAVORS: Rect = Rect::new(0, 0, 79, 1);
    pub(super) const VANILLA: Rect = Rect::new(OPTION_INDENT, 1, OPTION_HIGHLIGHT_WIDTH, 1);
    pub(super) const CHOCOLATE: Rect = Rect::new(OPTION_INDENT, 2, OPTION_HIGHLIGHT_WIDTH, 1);
    pub(super) const STRAWBERRY: Rect = Rect::new(OPTION_INDENT, 3, OPTION_HIGHLIGHT_WIDTH, 1);
    pub(super) const CINNAMON: Rect = Rect::new(OPTION_INDENT, 4, OPTION_HIGHLIGHT_WIDTH, 1);
    //pub(super) const BUTTERSCOTCH: Rect = Rect::new(OPTION_INDENT, 5, OPTION_HIGHLIGHT_WIDTH, 1);
    //pub(super) const PEANUT_BUTTER_FUDGE: Rect =
    //    Rect::new(OPTION_INDENT, 6, OPTION_HIGHLIGHT_WIDTH, 1);
    pub(super) const CHILI: Rect = Rect::new(OPTION_INDENT, 7, OPTION_HIGHLIGHT_WIDTH, 1);

    pub(super) const TOPPINGS: Rect = Rect::new(0, 9, 79, 1);
    pub(super) const WHIPPED_CREAM: Rect = Rect::new(OPTION_INDENT, 10, OPTION_HIGHLIGHT_WIDTH, 1);
    pub(super) const HOT_FUDGE: Rect = Rect::new(OPTION_INDENT, 11, OPTION_HIGHLIGHT_WIDTH, 1);
    //pub(super) const NUTS: Rect = Rect::new(OPTION_INDENT, 12, OPTION_HIGHLIGHT_WIDTH, 1);
    //pub(super) const CHERRY: Rect = Rect::new(OPTION_INDENT, 13, OPTION_HIGHLIGHT_WIDTH, 1);
    pub(super) const BANANA: Rect = Rect::new(OPTION_INDENT, 14, OPTION_HIGHLIGHT_WIDTH, 1);

    pub(super) const OK: Rect = Rect::new(27, 16, 4, 1);
    pub(super) const CANCEL: Rect = Rect::new(46, 16, 8, 1);
}

#[test]
fn draw_flavors() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
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
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
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
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
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

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHOCOLATE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHOCOLATE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
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

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHOCOLATE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHOCOLATE, HIGHLIGHT_STYLE);
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

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHOCOLATE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHOCOLATE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CINNAMON, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CINNAMON, HIGHLIGHT_STYLE);
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

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::HOT_FUDGE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::HOT_FUDGE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::BANANA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::BANANA, HIGHLIGHT_STYLE);
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

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::HOT_FUDGE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::HOT_FUDGE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Up.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
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

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn shift_tab_around() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    assert_eq!(
        app.focus,
        Focus::Item {
            list: 0,
            option: 0,
            index: 1
        }
    );
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_tab_shift_tab() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CINNAMON, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_shift_tab_tab() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CINNAMON, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::BackTab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_all_the_way() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    for _ in 0..12 {
        app.handle_event(Event::Key(KeyCode::Down.into()));
        assert!(app.get_output().is_none());
    }
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Left.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn goto_bottom() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_then_goto_top() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::STRAWBERRY, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_tab_then_goto_top() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::STRAWBERRY, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn goto_top_from_ok() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::STRAWBERRY, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn goto_top_from_cancel() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::STRAWBERRY, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Home.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
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

    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
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

    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::VANILLA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHOCOLATE, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Down.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Enter.into()));
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::HOT_FUDGE, HIGHLIGHT_STYLE);
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

#[test]
fn up_a_list() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::Tab.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::WHIPPED_CREAM, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Up.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CHILI, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn up_from_ok() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::OK, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Up.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::BANANA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn up_from_cancel() {
    let mut form = Form::new();
    form.add("flavor", RadioSelector::new("Flavors:", FLAVORS));
    form.add("toppings", MultiSelector::new("Toppings:", TOPPINGS));
    let mut app = App::from(form);
    assert!(app.get_output().is_none());

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    app.handle_event(Event::Key(KeyCode::Right.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::CANCEL, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);

    app.handle_event(Event::Key(KeyCode::Up.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
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
        "                           <OK>               <Cancel>                          ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
        "                                                                                ",
    ]);
    expected.set_style(areas::FLAVORS, TITLE_STYLE);
    expected.set_style(areas::TOPPINGS, TITLE_STYLE);
    expected.set_style(areas::BANANA, HIGHLIGHT_STYLE);
    pretty_assertions::assert_eq!(buffer, expected);
}
