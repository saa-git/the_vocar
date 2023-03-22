#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::{
    Screen::{
        self,
        *
    },
    Vocar
};

mod demographic;
pub use demographic::{
    Demo,
    Class::{self, *},
    Race::{self, *}
};