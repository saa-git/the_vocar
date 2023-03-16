#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::{Screen::{self, *}, VocarApp};

mod demo;
pub use demo::{
    Demo,
    Class::{self, *},
    Race::{self, *}
};