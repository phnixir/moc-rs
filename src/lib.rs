#![doc = include_str!("../README.md")]

/// This module contains common enums and structs.
pub mod common;
/// The module containing all the useful stuff: the `Moc` struct and the most important trait
/// in the whole crate
pub mod moc;

pub use common::{MocControl, MocInfo, MocSource, MocState};
pub use moc::{Moc, MocInterface};
