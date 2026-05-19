fn main() {
    let s = String::from("hola mundo");

    primera_copia(s);

    let x = 5;
    segunda_copia(x);
    println!("numero {x}");
}

fn primera_copia(un_string: String) {
    println!("{un_string}");
}

fn segunda_copia(un_entero: i32) {
    println!("{un_entero}");
}
