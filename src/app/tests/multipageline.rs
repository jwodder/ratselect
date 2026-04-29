// IMPORTANT: In these tests, `app.render()` needs to be called before the
// first call to `app.handle_event()` so that the `App` fills in the screen
// height in the former call for use in the latter.
use crate::app::*;
use ratatui::{buffer::Buffer, layout::Rect};

const LOREM: [&str; 7] = [
    "Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod",
    "tincidunt ut laoreet dolore magna aliquam erat volutpat.  Ut wisi enim ad minim veniam,",
    "quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo",
    "consequat.  Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse",
    "molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan",
    "et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore",
    "te feugait nulla facilisi.  Nam liber tempor cum soluta nobis eleifend option congue",
];

const IPSUM: [&str; 7] = [
    "nihil imperdiet doming id quod mazim placerat facer possim assum.  Typi non habent",
    "claritatem insitam; est usus legentis in iis qui facit eorum claritatem.  Investigationes",
    "demonstraverunt lectores legere me lius quod ii legunt saepius.  Claritas est etiam",
    "processus dynamicus, qui sequitur mutationem consuetudium lectorum.  Mirum est notare",
    "quam littera gothica, quam nunc putamus parum claram, anteposuerit litterarum formas",
    "humanitatis per seacula quarta decima et quinta decima.  Eodem modo typi, qui nunc",
    "nobis videntur parum clari, fiant sollemnes in futurum.",
];

mod areas {
    use super::*;

    pub(super) const SCREEN: Rect = Rect::new(0, 0, 80, 24);

    //pub(super) const OK: Rect = Rect::new(27, 23, 4, 1);
    //pub(super) const CANCEL: Rect = Rect::new(46, 23, 8, 1);
}

fn mkform() -> Form<&'static str> {
    let mut form = Form::new();
    form.add(
        "foo",
        RadioSelector::new(
            "Foo",
            [
                "Foo\nFoo\nFoo",
                "Foo Bar\nFoo Bar",
                "Foo Quux\nFoo Quux\nFoo Quux\nFoo Quux",
                "Foo Gnusto\nFoo Gnusto",
            ],
        ),
    );
    form.add(
        "bar",
        RadioSelector::new(
            "Bar",
            [
                "Bar\nBar\nBar",
                "Bar Foo\nBar Foo\nBar Foo",
                "Bar Baz\nBar Baz",
                "Bar Quux\nBar Quux\nBar Quux\nBar Quux",
                "Bar Gnusto\nBar Gnusto",
            ],
        ),
    );
    form.add(
        "baz",
        RadioSelector::new(
            "Baz",
            [
                "Baz\nBaz\nBaz",
                "Baz Foo\nBaz Foo\nBaz Foo",
                "Baz Bar\nBaz Bar",
                "Baz Quux\nBaz Quux\nBaz Quux\nBaz Quux",
                "Baz Gnusto\nBaz Gnusto",
            ],
        ),
    );
    form.add(
        "quux",
        RadioSelector::new(
            "Quux",
            [
                "Quux\nQuux\nQuux",
                "Quux Foo\nQuux Foo\nQuux Foo",
                "Quux Bar\nQuux Bar",
                "Quux Baz\nQuux Baz\nQuux Baz\nQuux Baz",
                "Quux Gnusto\nQuux Gnusto",
            ],
        ),
    );
    form
}

fn mklorem() -> Form<&'static str> {
    let mut form = Form::new();
    form.add("lorem", RadioSelector::new("Lorem", LOREM));
    form.add("ipsum", MultiSelector::new("Ipsum", IPSUM));
    form
}

#[test]
fn draw() {
    let mut app = App::from(mkform());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Foo                                                                            ▲",
        "    (X) Foo                                                                    █",
        "        Foo                                                                    █",
        "        Foo                                                                    █",
        "    ( ) Foo Bar                                                                █",
        "        Foo Bar                                                                █",
        "    ( ) Foo Quux                                                               █",
        "        Foo Quux                                                               █",
        "        Foo Quux                                                               █",
        "        Foo Quux                                                               █",
        "    ( ) Foo Gnusto                                                             ▒",
        "        Foo Gnusto                                                             ▒",
        "                                                                               ▒",
        "Bar                                                                            ▒",
        "    (X) Bar                                                                    ▒",
        "        Bar                                                                    ▒",
        "        Bar                                                                    ▒",
        "    ( ) Bar Foo                                                                ▒",
        "        Bar Foo                                                                ▒",
        "        Bar Foo                                                                ▒",
        "    ( ) Bar Baz                                                                ▒",
        "        Bar Baz                                                                ▒",
        "    ( ) Bar Quux                                                               ▒",
        "        Bar Quux                                                               ▼",
    ]);
    expected.set_style(Rect::new(0, 0, 79, 1), TITLE_STYLE); // "Foo"
    expected.set_style(Rect::new(0, 13, 79, 1), TITLE_STYLE); // "Bar"
    expected.set_style(Rect::new(4, 1, 15, 3), HIGHLIGHT_STYLE); // "Foo Foo Foo"
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn down_to_split_bottom() {
    let mut app = App::from(mkform());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);

    for _ in 0..7 {
        app.handle_event(Event::Key(KeyCode::Down.into()));
        assert!(app.get_output().is_none());
    }
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "    ( ) Foo Bar                                                                ▲",
        "        Foo Bar                                                                ▒",
        "    ( ) Foo Quux                                                               █",
        "        Foo Quux                                                               █",
        "        Foo Quux                                                               █",
        "        Foo Quux                                                               █",
        "    ( ) Foo Gnusto                                                             █",
        "        Foo Gnusto                                                             █",
        "                                                                               █",
        "Bar                                                                            █",
        "    (X) Bar                                                                    █",
        "        Bar                                                                    ▒",
        "        Bar                                                                    ▒",
        "    ( ) Bar Foo                                                                ▒",
        "        Bar Foo                                                                ▒",
        "        Bar Foo                                                                ▒",
        "    ( ) Bar Baz                                                                ▒",
        "        Bar Baz                                                                ▒",
        "    ( ) Bar Quux                                                               ▒",
        "        Bar Quux                                                               ▒",
        "        Bar Quux                                                               ▒",
        "        Bar Quux                                                               ▒",
        "    ( ) Bar Gnusto                                                             ▒",
        "        Bar Gnusto                                                             ▼",
    ]);
    expected.set_style(Rect::new(0, 9, 79, 1), TITLE_STYLE); // "Bar"
    expected.set_style(Rect::new(4, 18, 15, 4), HIGHLIGHT_STYLE); // "Bar Quux" x 4
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn draw_lorem() {
    let mut app = App::from(mklorem());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "Lorem                                                                          ▲",
        "    (X) Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam     █",
        "        nonummy nibh euismod                                                   █",
        "    ( ) tincidunt ut laoreet dolore magna aliquam erat volutpat.  Ut wisi enim █",
        "        ad minim veniam,                                                       █",
        "    ( ) quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut       █",
        "        aliquip ex ea commodo                                                  █",
        "    ( ) consequat.  Duis autem vel eum iriure dolor in hendrerit in vulputate  █",
        "        velit esse                                                             █",
        "    ( ) molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero█",
        "        eros et accumsan                                                       █",
        "    ( ) et iusto odio dignissim qui blandit praesent luptatum zzril delenit    █",
        "        augue duis dolore                                                      █",
        "    ( ) te feugait nulla facilisi.  Nam liber tempor cum soluta nobis eleifend █",
        "        option congue                                                          █",
        "                                                                               █",
        "Ipsum                                                                          █",
        "    [ ] nihil imperdiet doming id quod mazim placerat facer possim assum.  Typi█",
        "        non habent                                                             ▒",
        "    [ ] claritatem insitam; est usus legentis in iis qui facit eorum           ▒",
        "        claritatem.  Investigationes                                           ▒",
        "    [ ] demonstraverunt lectores legere me lius quod ii legunt saepius.        ▒",
        "        Claritas est etiam                                                     ▒",
        "    [ ] processus dynamicus, qui sequitur mutationem consuetudium lectorum.    ▼",
    ]);
    expected.set_style(Rect::new(0, 0, 79, 1), TITLE_STYLE); // "Lorem"
    expected.set_style(Rect::new(0, 16, 79, 1), TITLE_STYLE); // "Ipsum"
    expected.set_style(Rect::new(4, 1, 75, 2), HIGHLIGHT_STYLE); // "Lorem ipsum ..."
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn lorem_down_to_split_bottom() {
    let mut app = App::from(mklorem());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);

    for _ in 0..10 {
        app.handle_event(Event::Key(KeyCode::Down.into()));
        assert!(app.get_output().is_none());
    }
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "    (X) Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam     ▲",
        "        nonummy nibh euismod                                                   ▒",
        "    ( ) tincidunt ut laoreet dolore magna aliquam erat volutpat.  Ut wisi enim █",
        "        ad minim veniam,                                                       █",
        "    ( ) quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut       █",
        "        aliquip ex ea commodo                                                  █",
        "    ( ) consequat.  Duis autem vel eum iriure dolor in hendrerit in vulputate  █",
        "        velit esse                                                             █",
        "    ( ) molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero█",
        "        eros et accumsan                                                       █",
        "    ( ) et iusto odio dignissim qui blandit praesent luptatum zzril delenit    █",
        "        augue duis dolore                                                      █",
        "    ( ) te feugait nulla facilisi.  Nam liber tempor cum soluta nobis eleifend █",
        "        option congue                                                          █",
        "                                                                               █",
        "Ipsum                                                                          █",
        "    [ ] nihil imperdiet doming id quod mazim placerat facer possim assum.  Typi█",
        "        non habent                                                             █",
        "    [ ] claritatem insitam; est usus legentis in iis qui facit eorum           ▒",
        "        claritatem.  Investigationes                                           ▒",
        "    [ ] demonstraverunt lectores legere me lius quod ii legunt saepius.        ▒",
        "        Claritas est etiam                                                     ▒",
        "    [ ] processus dynamicus, qui sequitur mutationem consuetudium lectorum.    ▒",
        "        Mirum est notare                                                       ▼",
    ]);
    expected.set_style(Rect::new(0, 15, 79, 1), TITLE_STYLE); // "Ipsum"
    expected.set_style(Rect::new(4, 22, 75, 2), HIGHLIGHT_STYLE); // "processus dynamicus ..."
    pretty_assertions::assert_eq!(buffer, expected);
}

#[test]
fn lorem_goto_bottom() {
    let mut app = App::from(mklorem());
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);

    app.handle_event(Event::Key(KeyCode::End.into()));
    assert!(app.get_output().is_none());
    let mut buffer = Buffer::empty(areas::SCREEN);
    app.render(areas::SCREEN, &mut buffer);
    let mut expected = Buffer::with_lines([
        "    ( ) molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero▲",
        "        eros et accumsan                                                       ▒",
        "    ( ) et iusto odio dignissim qui blandit praesent luptatum zzril delenit    ▒",
        "        augue duis dolore                                                      ▒",
        "    ( ) te feugait nulla facilisi.  Nam liber tempor cum soluta nobis eleifend ▒",
        "        option congue                                                          ▒",
        "                                                                               ▒",
        "Ipsum                                                                          █",
        "    [ ] nihil imperdiet doming id quod mazim placerat facer possim assum.  Typi█",
        "        non habent                                                             █",
        "    [ ] claritatem insitam; est usus legentis in iis qui facit eorum           █",
        "        claritatem.  Investigationes                                           █",
        "    [ ] demonstraverunt lectores legere me lius quod ii legunt saepius.        █",
        "        Claritas est etiam                                                     █",
        "    [ ] processus dynamicus, qui sequitur mutationem consuetudium lectorum.    █",
        "        Mirum est notare                                                       █",
        "    [ ] quam littera gothica, quam nunc putamus parum claram, anteposuerit     █",
        "        litterarum formas                                                      █",
        "    [ ] humanitatis per seacula quarta decima et quinta decima.  Eodem modo    █",
        "        typi, qui nunc                                                         █",
        "    [ ] nobis videntur parum clari, fiant sollemnes in futurum.                █",
        "                                                                               █",
        "                           <OK>               <Cancel>                         █",
        "                                                                               ▼",
    ]);
    expected.set_style(Rect::new(0, 7, 79, 1), TITLE_STYLE); // "Ipsum"
    expected.set_style(Rect::new(27, 22, 4, 1), HIGHLIGHT_STYLE); // "<OK>"
    pretty_assertions::assert_eq!(buffer, expected);
}
