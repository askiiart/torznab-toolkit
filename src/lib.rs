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

mod api;
pub mod config;
pub mod data;

use config::{Caps, Config};
use rocket;

pub fn run(config: Config, caps: Caps) -> Result<bool, String> {
    /// Runs the server
    rocket::build().mount("/", rocket::routes![api::caps]);
    return Ok(true);
}
