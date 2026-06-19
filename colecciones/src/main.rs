fn main() {
    //let v = vec![1, 2, 3, 4, 5];

    //let does_not_exist = &v[100];
    //let does_not_exist = v.get(100); // dado que esto de aqui fallaría por limite de index
    // no mostrará nada de las variables creadas posteriormente en los strings

    let s1 = String::from("hello");
    let s2 = String::from(", world!");
    let s3 = s1 + &s2;
    let print = format!("s3: {s3}");

    println!("{print}"); // s3: hello, world! 

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}")
    // Esto funciona porque el metodo push_str permite añadir la referencia de un &str literal a un tipo String sin embargo al reves no.
}
