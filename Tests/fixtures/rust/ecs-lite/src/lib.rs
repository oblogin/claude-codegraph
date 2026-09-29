//! A tiny entity-component toolkit used as a code-graph fixture.

#[macro_use]
mod macros;

pub mod components;
pub mod math;
pub mod render;
pub mod ui;
pub mod world;

pub use render::{Draw, Renderer};
pub use world::World;
