use std::{fs::File, io::ErrorKind, net::IpAddr};

fn main() {
    let archivo = File::open("Hello.txt");
    let _result = match archivo {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Ha habido un error: {e:?}"),
            },
            _ => {
                panic!("Problema al abrir el archivo {error:?}");
            }
        },
    };
    let ip: IpAddr = "127.0.0.1".parse().expect("nopee");
    format!("ip: {ip}");
}
