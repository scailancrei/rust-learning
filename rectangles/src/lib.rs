#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_can_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
    }
}