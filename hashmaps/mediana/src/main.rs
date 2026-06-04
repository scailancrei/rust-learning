use std::collections::HashMap;
fn main() {
    let mut v = vec![4, 7, 2, 9, 7, 3, 1, 7, 5, 2, 8, 4, 7];
    let mut map = HashMap::new();

    v.sort();
    let position = v.len() / 2;
    let mediana: i32 = v[position];
    for numbers in v {
        let count = map.entry(numbers).or_insert(0);
        *count += 1;
    }
    let moda: Option<(&i32, &i32)> = map.iter().max_by_key(|(_clave, valor)| **valor);

    match moda {
        Some((numero, _)) => println!("la moda es: {numero:?}"),
        None => println!("no hay valor"),
    }

    println!("La mediana es {mediana}");
}
