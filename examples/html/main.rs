use scraper::Html;

fn main() {
    let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

    let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");

    println!("{:#?}", fragment);
}
