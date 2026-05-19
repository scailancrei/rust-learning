fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} y {r2}");

    let r3 = &mut s;
    println!("{r3}, {r1}"); // esto fallaria porque r1 es un referencia inmutable
    // si una referencia inmutable ha sido usada antes de declarar una mutable esta deja de ser valida y no lograria encontrar su referencia en memoria
}
