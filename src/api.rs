//! Contains the actual Torznab API
use std::io::stdout;

use crate::data::*;
use crate::dummy::create_empty_config;
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::response::status;
use rocket::{get, response::content::RawXml};
use xml::writer::{EmitterConfig, XmlEvent};

// Holds the config for torznab-toolkit.
//
// A search function (`/api?t=search`) and capabilities (`/api?t=caps` - `Caps`) are required, everything else is optional.
//
// <div class="warning">It's required to be set to <i>something</i>, which is why it's an Option set to None.
//
// However, this is NOT optional, and attempting to do anything with CONFIG not set will return an `Err`.</div>

pub(crate) static mut CONFIG: Option<Config> = None;
lazy_static! {
    static ref STATUS_CONFIG_NOT_SPECIFIED: status::Custom<RawXml<String>> = status::Custom(
        Status::InternalServerError,
        RawXml("500 Internal server error: Config not specified".to_string()),
    );
}

/// Capabilities API endpoint (`/api?t=caps`)
///
/// Note that an apikey is *not* required for this function, regardless of whether it's required for the rest.
// FIXME: VERY incomplete
#[get("/api?t=caps")]
pub(crate) fn caps() -> status::Custom<RawXml<String>> {
    // The compiler won't let you get a field from a struct in the Option here, since the default is None
    // So this is needed
    let conf = create_empty_config();
    unsafe {
        if CONFIG.is_none() {
            return (*STATUS_CONFIG_NOT_SPECIFIED).clone();
        } else {
            let conf: Config = CONFIG.clone().ok_or("").unwrap();
        }
    }

    let output = stdout();
    let mut writer = EmitterConfig::new().create_writer(output);

    writer.write(XmlEvent::start_element("caps")).unwrap();
    writer.write(XmlEvent::start_element("server")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::start_element("caps")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    return status::Custom(Status::Ok, RawXml(stringify!(writer).to_string()));
}

#[get("/api?t=search&<form..>")]
pub(crate) fn search(form: SearchForm) -> status::Custom<RawXml<String>> {
    // The compiler won't let you get a field from a struct in the Option here, since the default is None
    // So this is needed
    let conf = create_empty_config();
    unsafe {
        if CONFIG.is_none() {
            return (*STATUS_CONFIG_NOT_SPECIFIED).clone();
        } else {
            let conf: Config = CONFIG.clone().ok_or("").unwrap();
        }
    }

    // TODO: Clean up this code - split it into a separate function?
    let mut apikey: String = "".to_string();
    if !form.apikey.is_none() {
        apikey = form.apikey.ok_or("").unwrap();
    }

    let mut categories: Vec<u32> = Vec::new();
    if !form.cat.is_none() {
        // unholy amalgation of code to make the comma-separated list of strings into a vector of integers
        categories = form
            .cat
            .ok_or("")
            .unwrap()
            .split(",")
            .filter_map(|s| s.parse().ok())
            .collect();
    }

    let mut extended_attribute_names: String = "".to_string();
    if !form.attrs.is_none() {
        extended_attribute_names = form.attrs.ok_or("").unwrap().split(",").collect();
    }

    let mut extended_attrs: bool = false;
    if !form.extended.is_none() && form.extended.ok_or(false).unwrap() == 1 {
        extended_attrs = true;
    }

    let mut offset: u32 = 0;
    if !form.offset.is_none() {
        offset = form.offset.ok_or(0).unwrap();
    }

    let mut limit: u32 = 0;
    limit = conf.caps.limits.max;
    let wanted_limit = form.limit.ok_or(limit).unwrap();
    if wanted_limit < limit {
        limit = wanted_limit
    }

    match conf.auth {
        Some(auth) => {
            if !auth(apikey).unwrap() {
                return status::Custom(
                    Status::Unauthorized,
                    RawXml("401 Unauthorized".to_string()),
                );
            }
        }
        None => {}
    }

    return status::Custom(
        Status::NotImplemented,
        RawXml("501 Not Implemented: Search function not implemented".to_string()),
    );
}
