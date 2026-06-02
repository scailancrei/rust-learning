#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_coin(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} old"))
    } else {
        Some(format!("{state:?} new"))
    }
}

fn main() {
    if let Some(state) = describe_coin(Coin::Quarter(UsState::Alabama)) {
        println!("BIENVENIDO A {state:?}");
    }
}
