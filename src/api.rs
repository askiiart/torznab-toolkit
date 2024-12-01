//! Contains the actual Torznab API
use crate::data::*;
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::response::status;
use rocket::FromForm;
use rocket::{get, response::content::RawXml};
use std::str;
use xml::writer::{EmitterConfig, XmlEvent};

#[derive(Debug, Clone, PartialEq, Eq, FromForm)]
/// A struct used by the API's search functions to hold its query parameters
/// Currently required (AFAIK) because of limitations with rocket
struct SearchForm {
    /// The text query for the search
    q: Option<String>,
    /// The apikey, for authentication
    apikey: Option<String>,
    /// The list of numeric category IDs to be included in the search results
    /// Returned by Rocket.rs as a string of comma-separated values, then split in the function to a `Vec<u32>`
    cat: Option<String>,
    /// The list of extended attribute names to be included in the search results
    /// Returned by Rocket.rs as a string of comma-separated values, then split in the function to a `Vec<String>`
    attrs: Option<String>,
    /// Whether *all* extended attributes should be included in the search results; overrules `attrs`
    /// Can be 0 or 1
    extended: Option<u8>,
    /// How many items to skip/offset by in the results.
    offset: Option<u32>,
    /// The maximum number of items to return - also limited to whatever `limits` is in [`Caps`]
    limit: Option<u32>,
}

impl SearchForm {
    /// Converts it to a SearchParameters object
    fn to_parameters(&self, conf: Config) -> InternalSearchParameters {
        // TODO: Clean up this code - split it into a separate function?
        let mut categories: Option<Vec<u32>> = None;
        if !self.cat.is_none() {
            // unholy amalgation of code to make the comma-separated list of strings into a vector of integers
            categories = Some(
                self.cat
                    .as_ref()
                    .ok_or("")
                    .unwrap()
                    .split(",")
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            );
        }

        let mut extended_attribute_names: Option<Vec<String>> = None;
        if !self.attrs.is_none() {
            extended_attribute_names = Some(
                self.attrs
                    .as_ref()
                    .ok_or("")
                    .unwrap()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect(),
            );
        }

        let mut extended_attrs: Option<bool> = None;
        if !self.extended.is_none() && self.extended.ok_or(false).unwrap() == 1 {
            extended_attrs = Some(true);
        }

        let mut limit: u32 = self.limit.ok_or("").unwrap_or(conf.caps.limits.max);
        if limit > conf.caps.limits.max {
            limit = conf.caps.limits.max;
        }

        return InternalSearchParameters {
            q: self.q.clone(),
            apikey: self.apikey.clone(),
            categories: categories,
            attributes: extended_attribute_names,
            extended_attrs: extended_attrs,
            offset: self.offset,
            limit: limit,
        };
    }
}

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
#[get("/api?t=caps")]
pub(crate) fn caps() -> status::Custom<RawXml<String>> {
    // The compiler won't let you get a field from a struct in the Option here, since the default is None
    // So this is needed
    let conf;
    unsafe {
        if CONFIG.is_none() {
            return (*STATUS_CONFIG_NOT_SPECIFIED).clone();
        } else {
            conf = CONFIG.clone().ok_or("").unwrap();
        }
    }

    let buffer = Vec::new();
    let mut writer = EmitterConfig::new().create_writer(buffer);

    writer.write(XmlEvent::start_element("caps")).unwrap();

    // add the server info
    // TODO: Clean up the code by making the elements a Vec (to be used as a stack), rather than manually keeping track of them
    let mut element = XmlEvent::start_element("server");
    match &conf.caps.server_info {
        Some(server_info) => {
            // needs to be a vec since if i just `.as_str()` them, they don't live long enough
            let server_info_vec: Vec<(&String, &String)> = server_info.iter().collect();
            for (key, value) in server_info_vec {
                element = element.attr(key.as_str(), value.as_str());
            }
        }
        None => {}
    }
    writer.write(element).unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // close `server`

    // add the limits
    writer
        .write(
            XmlEvent::start_element("limits")
                .attr("max", conf.caps.limits.max.to_string().as_str())
                .attr("default", conf.caps.limits.default.to_string().as_str()),
        )
        .unwrap();
    writer.write(XmlEvent::end_element()).unwrap(); // close `limits`

    // Add the search types
    writer.write(XmlEvent::start_element("searching")).unwrap();
    for item in conf.caps.searching {
        let mut available = "yes";
        if !item.available {
            available = "no";
        }
        writer
            .write(
                XmlEvent::start_element(item.search_type.as_str())
                    .attr("available", available)
                    .attr("supportedParams", item.supported_params.join(",").as_str()),
            )
            .unwrap();
        writer.write(XmlEvent::end_element()).unwrap(); // close element
    }
    writer.write(XmlEvent::end_element()).unwrap(); // close `searching`

    writer.write(XmlEvent::start_element("categories")).unwrap();
    for i in conf.caps.categories {
        writer
            .write(
                XmlEvent::start_element("category")
                    .attr("id", i.id.to_string().as_str())
                    .attr("name", i.name.as_str()),
            )
            .unwrap();
        for j in i.subcategories {
            writer
                .write(
                    XmlEvent::start_element("subcat")
                        .attr("id", j.id.to_string().as_str())
                        .attr("name", j.name.as_str()),
                )
                .unwrap();
            writer.write(XmlEvent::end_element()).unwrap(); // close `subcat` element
        }
        writer.write(XmlEvent::end_element()).unwrap(); // close `category` element
    }
    writer.write(XmlEvent::end_element()).unwrap(); // close `categories`

    match conf.caps.genres {
        Some(genres) => {
            writer.write(XmlEvent::start_element("genres")).unwrap();

            for genre in genres {
                writer
                    .write(
                        XmlEvent::start_element("genre")
                            .attr("id", genre.id.to_string().as_str())
                            .attr("categoryid", genre.category_id.to_string().as_str())
                            .attr("name", genre.name.as_str()),
                    )
                    .unwrap();
                writer.write(XmlEvent::end_element()).unwrap(); // close `genre` element
            }
            writer.write(XmlEvent::end_element()).unwrap(); // close `genres` element
        }
        None => {}
    }

    match conf.caps.tags {
        Some(tags) => {
            writer.write(XmlEvent::start_element("tags")).unwrap();

            for tag in tags {
                writer
                    .write(
                        XmlEvent::start_element("tag")
                            .attr("name", tag.name.as_str())
                            .attr("description", tag.description.as_str()),
                    )
                    .unwrap();
            }
            writer.write(XmlEvent::end_element()).unwrap(); // close `tags` element
        }
        None => {}
    }

    writer.write(XmlEvent::end_element()).unwrap(); // close `caps`
    let result = str::from_utf8(writer.into_inner().as_slice())
        .unwrap()
        .to_string(); // Convert buffer to a String

    return status::Custom(Status::Ok, RawXml(result));
}

#[get("/api?t=search&<form..>")]
/// The search function for the API
// FIXME: VERY incomplete also
pub(crate) fn search(form: SearchForm) -> status::Custom<RawXml<String>> {
    // The compiler won't let you get a field from a struct in the Option here, since the default is None
    // So this is needed
    let conf;
    unsafe {
        if CONFIG.is_none() {
            return (*STATUS_CONFIG_NOT_SPECIFIED).clone();
        } else {
            conf = CONFIG.clone().ok_or("").unwrap();
        }
    }

    let parameters = form.to_parameters(conf.clone());

    let mut unauthorized = false;
    match conf.auth {
        Some(auth) => {
            match parameters.apikey {
                Some(apikey) => {
                    if !auth(apikey).unwrap() {
                        unauthorized = true;
                    }
                }
                None => {
                    unauthorized = true;
                }
            }
            // that unwrap_or_else is to return "" if the apikey isn't specified
        }
        None => {}
    }

    if unauthorized {
        return status::Custom(Status::Unauthorized, RawXml("401 Unauthorized".to_string()));
    }

    return status::Custom(
        Status::NotImplemented,
        RawXml("501 Not Implemented: Search function not implemented".to_string()),
    );
}
