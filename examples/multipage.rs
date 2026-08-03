use ratselect::{Form, RadioSelector, Selection};

fn main() -> std::io::Result<()> {
    let mut app = Form::new();
    for (ch, qty) in std::iter::zip("ABCDE".chars(), 16..) {
        app.add(
            ch,
            RadioSelector::new(format!("List {ch}"), (0..qty).map(|i| format!("{ch}{i}"))),
        );
    }
    let selections = app.run()?;
    if std::env::args_os().nth(1).is_some_and(|s| s == "--debug") {
        println!("{selections:#?}");
    } else if let Some(selections) = selections {
        for (k, v) in selections {
            if let Selection::Radio(i) = v {
                println!("List {k}: {k}{i}");
            } else {
                println!("UNEXPECTED OUTPUT: ({k:?}, {v:?})");
            }
        }
    } else {
        println!("User quit");
    }
    Ok(())
}
