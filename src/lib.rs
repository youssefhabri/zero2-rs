#![allow(clippy::unreadable_literal)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate diesel;

pub mod checks;
pub mod client;
pub mod commands;
pub mod core;
pub mod db;
pub mod menu;
pub mod models;
pub mod monitors;

pub use client::Zero2Client;
