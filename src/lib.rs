//! Links:
//! - **[Tutorial](notes::tutorial)**
//! - [Minor implementation and usage notes](notes::notes)
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
pub(crate) mod api;
pub mod data;
mod dummy;

use rocket;
// imports for docs
use crate::data::Config;

/// Runs the server
///
/// Returns `Ok(true)` if it succeeds, otherwise returns the error from Rocket
pub async fn run(conf: data::Config) -> Result<bool, rocket::Error> {
    match rocket::build()
        .mount(
            "/",
            rocket::routes![
                api::caps,
                api::search,
                api::tv_search,
                api::movie_search,
                api::music_search,
                api::book_search
            ],
        )
        .manage(conf)
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
