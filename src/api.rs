//! Contains the actual Torznab API
use std::io::stdout;

use crate::data::*;
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use xml::writer::{EmitterConfig, XmlEvent};

/// Holds the config for torznab-toolkit.
///
/// A search function (`/api?t=search`) and capabilities (`/api?t=caps` - `Caps`) are required, everything else is optional.
///
/// <div class="warning">It's required to be set to <i>something</i>, which is why it's an Option set to None.
///
/// However, this is NOT optional, and attempting to do anything with CONFIG not set will return an `Err`.</div>
pub static mut CONFIG: Option<Config> = None;

/// Capabilities API endpoint (`/api?t=caps`)
// FIXME: VERY incomplete
// TODO: Finish it (duh) and add optional apikey
#[get("/api?t=caps")]
pub(crate) fn caps() -> status::Custom<String> {
    unsafe {
        match CONFIG {
            None => {
                return status::Custom(
                    Status::InternalServerError,
                    "500 Internal server error: Config not specified".to_string(),
                );
            }
            Some(_) => {}
        }
    }

    let output = stdout();
    let mut writer = EmitterConfig::new().create_writer(output);

    writer.write(XmlEvent::start_element("caps")).unwrap();
    writer.write(XmlEvent::start_element("server")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    writer.write(XmlEvent::start_element("caps")).unwrap();
    writer.write(XmlEvent::end_element()).unwrap();
    return status::Custom(Status::Ok, stringify!(writer).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_search_func(a: String, b: Vec<String>) -> Result<String, String> {
        return Ok("hi".to_string());
    }

    fn dummy_auth_func(a: String) -> Result<bool, String> {
        return Ok(true);
    }

    fn create_config() -> Config {
        let mut searching = Vec::new();
        searching.push(SearchInfo {
            search_type: "search".to_string(),
            available: true,
            supported_params: vec!["id".to_string()],
        });

        let mut subcategories = Vec::new();
        subcategories.push(Subcategory {
            id: "a".to_string(),
            name: "b".to_string(),
        });

        let mut categories = Vec::new();
        categories.push(Category {
            id: "a".to_string(),
            name: "b".to_string(),
            subcategories: subcategories,
        });

        let mut genres = Vec::new();
        genres.push(Genre {
            id: "a".to_string(),
            category_id: "b".to_string(),
            name: "c".to_string(),
        });

        let mut tags = Vec::new();
        tags.push(Tag {
            id: "a".to_string(),
            category_id: "b".to_string(),
            name: "c".to_string(),
        });

        return Config {
            search: dummy_search_func,
            auth: Some(dummy_auth_func),
            caps: Caps {
                server_info: ServerInfo {
                    title: Some("Test Torznab server".to_string()),
                    email: Some("test@example.com".to_string()),
                    image: None,
                    version: Some("1.0".to_string()),
                },
                limits: Limits {
                    max: 100,
                    default: 20,
                },
                searching: searching,
                categories: categories,
                genres: Some(genres),
                tags: Some(tags),
            },
            book: None,
            movie: None,
            music: None,
            tvsearch: None,
        };
    }

    #[test]
    fn test_with_config() {
        unsafe {
            CONFIG = Some(create_config());
            println!("{:?}", CONFIG);
        }
        println!("{:?}", caps());
    }

    #[test]
    fn test_empty_config() {
        unsafe {
            println!("{:?}", CONFIG);
        }
        println!("{:?}", caps());
    }
}
