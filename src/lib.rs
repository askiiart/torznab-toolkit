#![warn(missing_docs)]
//! A toolkit for adding Torznab APIs to programs.
//!
//! Just fill in your own relevant functions and config, and
//! torznab-toolkit will run the API for you
//!
//! ```rs
//! use torznab_toolkit;
//! let config: torznab_toolkit::config::Config =
//!
//! ```
//!
//! The environment variables `ROCKET_ADDRESS` and `ROCKET_PORT` specify the address and port it will run on; these currently cannot be configured any other way. See the [relevant docs](https://rocket.rs/guide/v0.5/deploying/) for details.
//!
//! ---
//!
//! This program is brought to you by: metaphorical *and* literal truckloads of structs!
//!
//! Note: I wrote the line above when I was tired. Don't ask me what *literal* truckloads of structs means, I don't know either.

pub mod api;
pub mod data;

use rocket::{self};

pub fn run(conf: data::Config, caps: data::Caps) -> Result<bool, String> {
    /// Runs the server
    //rocket::build()
    //    .mount("/", rocket::routes![conf.caps])
    //    .launch();
    return Ok(true);
}
