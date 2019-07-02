//! Iota
//!
//! A highly customisable text editor built with modern hardware in mind.
//!
//! This module contains all you need to create an `iota` executable.

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(missing_docs)]

extern crate gapbuffer;
extern crate regex;
extern crate rustbox;
extern crate unicode_width;
#[macro_use]
extern crate lazy_static;

pub use editor::Editor;
pub use input::Input;

mod bindings;
mod buffer;
mod command;
mod editor;
mod input;
mod iterators;
mod key;
mod log;
mod textobject;
mod utils;
mod view;
