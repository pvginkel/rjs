//! # `rjs` JavaScript in Rust
//!
//! `rjs` is a JavaScript on Rust implementation.
//!
//! A simple usage of `rjs` is to instantiate a `JsEnv` and run either scripts
//! or code using it.
//!
//! To extend `rjs`, a reference to the global object can be retrieved through
//! the `JsEnv` instance. New objects can be made available to the script by
//! adding them to the global object.

// We only need the feature(test) attribute for test compilations. This is used
// for running benchmarks.
#![cfg_attr(test, feature(test))]

#[macro_use]
extern crate lazy_static;

pub use rt::{JsResult, JsError};

#[macro_use]
mod debug;
#[macro_use]
mod trace;
mod syntax;
mod ir;
mod util;
pub mod gc;
pub mod rt;
mod errors;
pub mod contrib;
