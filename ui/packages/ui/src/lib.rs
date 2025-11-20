//! This crate contains all shared UI for the workspace.

pub mod components;
pub mod io;
pub mod types;
pub mod views;
mod app;
mod navbar;

pub use app::{App, Route};
pub use navbar::Navbar;
