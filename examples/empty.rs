use ratselect::Form;

fn main() -> std::io::Result<()> {
    let app = Form::<i32>::new();
    let selections = app.run()?;
    println!("{selections:#?}");
    Ok(())
}
