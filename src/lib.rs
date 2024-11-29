#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
pub mod api;
pub mod data;
mod dummy;

use rocket::{self};

/// Runs the server
pub async fn run(conf: data::Config) -> Result<bool, rocket::Error> {
    unsafe {
        api::CONFIG = Some(conf);
    }
    match rocket::build()
        .mount("/", rocket::routes![api::caps, api::search])
        .launch()
        .await
    {
        Ok(_) => {
            return Ok(true);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

/// Notes regarding the usage of torznab-toolkit and how it implements the Torznab API.
pub mod notes;
