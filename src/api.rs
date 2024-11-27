//! Contains the actual Torznab API
use std::io::stdout;

use crate::data::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::{get, response::content::RawXml};
use xml::writer::{EmitterConfig, XmlEvent};

/// Holds the config for torznab-toolkit.
///
/// A search function (`/api?t=search`) and capabilities (`/api?t=caps` - `Caps`) are required, everything else is optional.
///
/// <div class="warning">It's required to be set to <i>something</i>, which is why it's an Option set to None.
///
/// However, this is NOT optional, and attempting to do anything with CONFIG not set will return an `Err`.</div>
pub(crate) static mut CONFIG: Option<Config> = None;

/// Capabilities API endpoint (`/api?t=caps`)
// FIXME: VERY incomplete
// TODO: Finish it (duh) and add optional apikey
#[get("/api?t=caps")]
pub(crate) fn caps() -> status::Custom<RawXml<String>> {
    unsafe {
        if CONFIG.is_none() {
            return status::Custom(
                Status::InternalServerError,
                RawXml("500 Internal server error: Config not specified".to_string()),
            );
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
