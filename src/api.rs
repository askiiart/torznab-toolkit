use crate::data::*;
use rocket::get;

/// A struct that holds configuration for torznab-toolkit
/// A search function (/api?t=search) and capabilities (/api?t=caps - struct Caps) required
/// Everything else is optional
pub static mut config: Option<Config> = None;
