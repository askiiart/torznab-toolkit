//! Contains the actual Torznab API
use crate::data::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::{get, response::content::RawXml};
use rocket::{FromForm, State};
use std::borrow::Borrow;
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
    fn to_parameters(
        &self,
        conf: impl Borrow<Config>,
        search_type: impl AsRef<str>,
    ) -> SearchParameters {
        let search_type: &str = search_type.as_ref();
        let conf: Config = conf.borrow().clone();

        let split = |string: String| -> Option<Vec<u32>> {
            Some(
                string
                    .split(",")
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            )
        };

        let categories = self.cat.clone().and_then(split);

        // also, let's just make it a closure at this point
        let extended_attribute_names: Option<Vec<String>> = self
            .attrs
            .clone()
            .and_then(|l| Some(l.split(",").map(|s| s.to_string()).collect()));

        // let mut extended_attrs: Option<bool> = self.extended.and_then(|k| if k == 1 { Some(true) } else { Some(false) }); // was this what you were trying to do?
        let mut extended_attrs = None;
        if !self.extended.is_none() && self.extended.ok_or(false).unwrap() == 1 {
            extended_attrs = Some(true);
        }

        let mut limit: u32 = self.limit.unwrap_or(conf.caps.limits.default);
        if limit > conf.caps.limits.max {
            limit = conf.caps.limits.max;
        }
        if limit < 1 {
            limit = 1
        }

        return SearchParameters {
            search_type: search_type.to_string(),
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

/// Capabilities API endpoint (`/api?t=caps`)
///
/// Note that an apikey is *not* required for this function, regardless of whether it's required for the rest.
#[get("/api?t=caps", rank = 1)]
pub(crate) async fn caps(conf: &State<Config>) -> status::Custom<RawXml<String>> {
    let buffer = Vec::new();
    let mut writer = EmitterConfig::new().create_writer(buffer);

    writer.write(XmlEvent::start_element("caps")).unwrap();

    // add the server info
    let mut element = XmlEvent::start_element("server");
    match &conf.caps.server_info {
        Some(server_info) => {
            // needs to be a vec since if i just `.as_str()` them, they don't live long enough
            let server_info_vec: Vec<(&String, &String)> = server_info.iter().collect();
            for (key, value) in server_info_vec {
                element = element.attr(key.as_str(), value);
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
    for item in &conf.caps.searching {
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
    for i in &conf.caps.categories {
        writer
            .write(
                XmlEvent::start_element("category")
                    .attr("id", i.id.to_string().as_str())
                    .attr("name", i.name.as_str()),
            )
            .unwrap();
        for j in &i.subcategories {
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

    match &conf.caps.genres {
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

    match &conf.caps.tags {
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

#[get("/api?t=search&<form..>", rank = 2)]
/// The general search function
pub(crate) async fn search(
    conf: &State<Config>,
    form: SearchForm,
) -> status::Custom<RawXml<String>> {
    // oh god this is horrible but it works
    let parameters = form.to_parameters((**conf).clone(), "search");

    let mut unauthorized = false;
    match conf.auth {
        Some(auth) => {
            match parameters.apikey.clone() {
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

    return search_handler(conf, parameters).await;
}

#[get("/api?t=tvsearch&<form..>", rank = 3)]
/// The TV search function
pub(crate) async fn tv_search(
    conf: &State<Config>,
    form: SearchForm,
) -> status::Custom<RawXml<String>> {
    // oh god this is horrible but it works
    let parameters = form.to_parameters((**conf).clone(), "tv-search");

    let mut unauthorized = false;
    match conf.auth {
        Some(auth) => {
            match parameters.clone().apikey {
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

    return search_handler(conf, parameters).await;
}

#[get("/api?t=movie&<form..>", rank = 4)]
/// The movie search function
pub(crate) async fn movie_search(
    conf: &State<Config>,
    form: SearchForm,
) -> status::Custom<RawXml<String>> {
    // oh god this is horrible but it works
    let parameters = form.to_parameters((**conf).clone(), "movie-search");

    let mut unauthorized = false;
    match conf.auth {
        Some(auth) => {
            match parameters.clone().apikey {
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

    return search_handler(conf, parameters).await;
}

#[get("/api?t=music&<form..>", rank = 5)]
/// The music search function
pub(crate) async fn music_search(
    conf: &State<Config>,
    form: SearchForm,
) -> status::Custom<RawXml<String>> {
    // oh god this is horrible but it works
    let parameters = form.to_parameters((**conf).clone(), "audio-search");

    let mut unauthorized = false;
    match conf.auth {
        Some(auth) => {
            match parameters.clone().apikey {
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

    return search_handler(conf, parameters).await;
}

#[get("/api?t=book&<form..>", rank = 6)]
/// The music search function
pub(crate) async fn book_search(
    conf: &State<Config>,
    form: SearchForm,
) -> status::Custom<RawXml<String>> {
    // oh god this is horrible but it works
    let parameters = form.to_parameters((**conf).clone(), "book-search");

    let mut unauthorized = false;
    match conf.auth {
        Some(auth) => {
            match parameters.clone().apikey {
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

    return search_handler(conf, parameters).await;
}

async fn search_handler(
    conf: &State<Config>,
    parameters: SearchParameters,
) -> status::Custom<RawXml<String>> {
    let buffer = Vec::new();
    let mut writer = EmitterConfig::new().create_writer(buffer);
    writer
        .write(
            XmlEvent::start_element("rss")
                .attr("version", "1.0")
                .attr("xmlns:atom", "http://www.w3.org/2005/Atom")
                .attr("xmlns:torznab", "http://torznab.com/schemas/2015/feed"),
        )
        .unwrap();
    writer.write(XmlEvent::start_element("channel")).unwrap();
    writer
        .write(
            XmlEvent::start_element("atom:link")
                .attr("rel", "self")
                .attr("type", "application/rss+xml"),
        )
        .unwrap();

    // add `title`
    writer.write(XmlEvent::start_element("title")).unwrap();
    let mut title_provided = false;
    match &conf.caps.server_info {
        Some(server_info) => {
            if server_info.contains_key("title") {
                match server_info.get("title") {
                    Some(title) => {
                        writer.write(XmlEvent::characters(title)).unwrap();
                        title_provided = true;
                    }
                    None => {}
                }
            }
        }
        None => {}
    }
    if !title_provided {
        writer
            .write(XmlEvent::characters("Torznab indexer"))
            .unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap();

    for item in (conf.search)(parameters).unwrap() {
        let torrent_file_url = item.torrent_file_url.clone().unwrap_or_default();

        let magnet_uri = item.magnet_uri.clone().unwrap_or_default();

        if torrent_file_url == "" && magnet_uri == "" {
            panic!("Torrent contains neither a .torrent file URL, not a magnet URI")
        }

        // start `item`
        writer.write(XmlEvent::start_element("item")).unwrap();

        // add `title`
        writer.write(XmlEvent::start_element("title")).unwrap();
        writer.write(XmlEvent::characters(&item.title)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // add `description`
        writer
            .write(XmlEvent::start_element("description"))
            .unwrap();
        if !item.description.is_none() {
            writer
                .write(XmlEvent::characters(&item.description.unwrap_or_default()))
                .unwrap();
        }
        writer.write(XmlEvent::end_element()).unwrap();

        // add `size` (torznab attr)
        writer
            .write(
                XmlEvent::start_element("torznab:attr")
                    .attr("size", item.size.to_string().as_str()),
            )
            .unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // add `category`s (torznab attr)
        for id in item.category_ids {
            writer
                .write(
                    XmlEvent::start_element("torznab:attr")
                        .attr("name", "category")
                        .attr("value", id.to_string().as_str()),
                )
                .unwrap();
            writer.write(XmlEvent::end_element()).unwrap();
        }

        // add `link` and `enclosure` (for torrent/magnet uri)
        // first check if `link` exists in hashmap, and if not, fallback to `torrent_file_url`, then `magnet_uri`
        writer.write(XmlEvent::start_element("link")).unwrap();
        let mut link_filled = false; // nesting two layers down of matches, so this is to keep track rather than just doing it in the None
        match item.other_attributes {
            Some(ref attributes) => match attributes.get("link") {
                Some(tmp) => {
                    writer.write(XmlEvent::characters(tmp)).unwrap();
                    link_filled = true;
                }
                None => {}
            },
            None => {}
        }

        if !link_filled {
            match item.torrent_file_url {
                Some(ref url) => {
                    writer.write(XmlEvent::characters(&url)).unwrap();
                    writer.write(XmlEvent::end_element()).unwrap();
                    writer
                        .write(
                            XmlEvent::start_element("enclosure")
                                .attr("url", &url)
                                .attr("length", 0.to_string().as_str())
                                .attr("type", "application/x-bittorrent"),
                        )
                        .unwrap();
                    writer.write(XmlEvent::end_element()).unwrap();
                }
                None => {
                    writer.write(XmlEvent::characters(&magnet_uri)).unwrap();
                    writer.write(XmlEvent::end_element()).unwrap();
                    writer
                        .write(
                            XmlEvent::start_element("enclosure")
                                .attr("url", &magnet_uri)
                                .attr("length", 0.to_string().as_str())
                                .attr("type", "application/x-bittorrent;x-scheme-handler/magnet"),
                        )
                        .unwrap();
                    writer.write(XmlEvent::end_element()).unwrap();
                }
            }
        }

        // add the remaining `other_attributes`
        match item.other_attributes {
            Some(ref other_attributes) => {
                for (key, value) in other_attributes {
                    writer
                        .write(XmlEvent::start_element("torznab::attr").attr(key.as_str(), value))
                        .unwrap();
                }
            }
            None => {}
        }

        writer.write(XmlEvent::end_element()).unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap(); // close `title`
    writer.write(XmlEvent::end_element()).unwrap(); // close `channel`
    writer.write(XmlEvent::end_element()).unwrap(); // close `rss`
    let result = str::from_utf8(writer.into_inner().as_slice())
        .unwrap()
        .to_string(); // Convert buffer to a String

    return status::Custom(Status::Ok, RawXml(result));
}
