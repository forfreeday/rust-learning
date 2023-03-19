//! # Art
//! A library for modeling artistic concepts.

pub mod kinds {
    #[derive(Debug)]
    pub enum PrimaryClolor {
        Red,
        Yellow,
        Blue,
    }
}

pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
}

pub mod utils {
    use crate::{kinds::*, SecondaryColor};

    pub fn mix(c1: PrimaryClolor, c2: PrimaryClolor) -> SecondaryColor {
        println!("c1 color is : {:#?}", c1);
        println!("c2 color is : {:#?}", c2);
        SecondaryColor::Green
    }
}
