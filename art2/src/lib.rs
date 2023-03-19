//! Art
//!
//! A library for modeling artistic concepts.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}


pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor{
        println!("c1 {:#?}", c1);
        println!("c2 {:#?}", c2);
        SecondaryColor::Green
    }
}
