//! ## Structure of the program using this library
//!
//! First off, you should create a search function. This function will handle all search types, with what type being specified in the parameter's `search_type` field (`search`, `tv-search`, `movie-search`, `audio-search`, or `movie-search`). Given those parameters, the search function then returns a [`Result`]<[`Vec`]<[`Torrent`]>, [`String`]> object.
//! The torrents will be listed by the API in the order they're returned here
//!
//! ```
//! use torznab_toolkit::data::{SearchParameters, Torrent};
//!
//! fn search(parameters: SearchParameters) -> Result<Vec<Torrent>, String> {
//!     return Ok(vec![Torrent {
//!         title: "totally normal torrent".to_string(),
//!         description: None,
//!         size: 2484345508,
//!         category_ids: vec![1010],
//!         torrent_file_url: Some("http://localhost/totally-normal.torrent".to_string()),
//!         magnet_uri: Some("magnet:?xt=urn:btih:blahblahblahdothechachacha".to_string()),
//!         other_attributes: None,
//!     }]);
//! }
//! ```
//!
//! If you want authentication, you can also create a function for that; returning true indicates that the apikey is valid.
//!
//! ```
//! fn auth(apikey: String) -> Result<bool, String> {
//!     if apikey == "letmein".to_string() {    
//!         return Ok(true);
//!     }
//!     return Ok(false);
//! }
//! ```
//!
//! Now you need to configure torznab-toolkit using a [`Config`] object. In total, you'll need the following objects for the config:
//! - The search function
//! - The API function (optional)
//! - The capabilities of the server - i.e.  ([`Caps`])
//!
//! Most of the config will be part of [`Caps`]. For details on all these, just check out the doc pages for each of the fields.
//!
//! With all that, you can now start up the server, which is simple:
//!
//! ```
//! use torznab_toolkit;
//! let config: torznab_toolkit::data::Config = /* config goes here */
//!
//! torznab_toolkit::run(config).await.unwrap();
//! ```
//!
//! To easily change what address is listens on and what port, you can use the `ROCKET_ADDRESS` and `ROCKET_PORT` environment variables; the defaults are `127.0.0.1` and `8000`.
//! For more details on configuring Rocket, see the [Configuration](https://rocket.rs/guide/v0.5/configuration/) page in Rocket's docs - you can also use a `Rocket.toml` file.

// imports for the docs
use crate::data::*;
use crate::run;
