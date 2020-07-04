#[macro_use]
extern crate nom;
pub mod parser;

pub fn deal() {
    println!("you get a hand!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_test() {
        assert!(true);
    }
}