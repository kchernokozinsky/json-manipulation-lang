use std::fmt::Display;

use crate::typings::Type;

pub mod integer;
pub mod null;
pub mod float;
pub mod bool;
pub mod list;

pub trait Value: Display {
    
    fn get_type(&self) -> Type;

}



