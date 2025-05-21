//! This crate contains all shared UI for the workspace.

pub mod views;
mod components;

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

pub mod recipe;