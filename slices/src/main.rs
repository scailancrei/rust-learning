fn main() {
    let mut s = String::from("hola mundo");

    let word = first_time_word(&s);
    s.clear();
    println!("{}", word);
    println!("cadena vacia {}", s); // si usamos la funcion first_time_word que en vez de devolver un slice de string devuelve
    // un usize la palbra word sigue teniendo el valor de 4 y la variable s es ahora vacia tras usar clear
    // sin embargo si cambiamos a la funcion first_word cambia radicalmente el comportamiento y
}

fn first_time_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // si el string pasado por parametro referenciado tiene un espacio, el for devuelve la posición del index
    // donde se encuentra dicho espacio
    // en caso de no tener espacio devuelve la longitud entera del string
    for (i, &item) in bytes.iter().enumerate() {
        //println!("{:?}", &item);
        if item == b' ' {
            return &s[0..i];
        }
    }
    println!("{}", s);
    &s[..]
}

fn slice_word() {
    let s = String::from("hello");

    let slice = &s[0..];
    println!("{}", slice)
}
