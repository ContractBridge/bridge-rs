use std::fmt;

#[macro_use]
extern crate nom;
pub mod parser;

pub fn deal() {
    println!("you get a hand!")
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_char(&self) -> char {
        match *self {
            Direction::North => 'N',
            Direction::East  => 'E',
            Direction::South => 'S',
            Direction::West  => 'W',
        }
    }
}

// Implementing Display trait on Direction
// https://stackoverflow.com/posts/32712140/revisions
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Player {
    pub direction: Direction,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        assert!(true);
    }

    #[test]
    fn test_direction_to_string() {
        assert_eq!("North".to_string(), Direction::North.to_string());
    }

    #[test]
    fn test_direction_to_char() {
        assert_eq!('N', Direction::North.to_char());
        assert_eq!('S', Direction::South.to_char());
        assert_eq!('E', Direction::East.to_char());
        assert_eq!('W', Direction::West.to_char());
    }

    #[test]
    fn test_player() {
        let p = Player {
            direction: Direction::West,
            name: "Bob".to_string()
        };

        println!("{:?}", p.direction);

        assert_eq!(String::from("Bob"), p.name);
        assert_eq!(Direction::West, p.direction);
    }
}