use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // println!("{field_name}");
    // println!("{field_value}"); // los campos field dejan de tener la propiedad debido a que son pasados en el insert por lo cual no son validos

    let text = "hello world wonderful world";

    let mut map1 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);

        *count += 1;
        println!("Map contains: {word} {count:?}");
    }

    println!("{map1:?}");
}
