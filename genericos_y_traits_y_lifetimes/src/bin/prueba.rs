mod lifetime;
use lifetime::Bookmark;
fn main() {
    let titulo1 = String::from("libro1");
    let titulo2 = String::from("libro2");
    let bookmark1 = Bookmark { title: titulo1 };
    let bookmark2 = Bookmark { title: titulo2 };
    let result;
    {
        result = get_major_title(bookmark1.title, bookmark2.title);
    }

    println!("resultado: {result}");
}

fn get_major_title<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
