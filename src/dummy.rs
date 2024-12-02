//! Some dummy stuff for testing the API
use crate::data::*;
use std::collections::HashMap;

#[macro_export]
/// HashMap equivalent of vec![]
///
/// Example:
/// ```rust
/// hashmap!(("key", "value"))
/// ```
macro_rules! hashmap {
    ($(($value1:expr, $value2:expr)),*) => {{
        let mut hm = HashMap::new();
        $(
            hm.insert($value1, $value2);
        )*

        hm
    }};
}

fn dummy_search_func(_a: SearchParameters) -> Result<Vec<Torrent>, String> {
    return Ok(vec![Torrent {
        title: "totally normal torrent".to_string(),
        description: None,
        size: 9872349573,
        category_ids: vec![1010],
        torrent_file_url: Some("http://localhost/totally-normal.torrent".to_string()),
        magnet_uri: Some("magnet:?xt=urn:btih:blahblahblahdothechachacha".to_string()),
        other_attributes: None,
    }]);
}

fn dummy_auth_func(_a: String) -> Result<bool, String> {
    return Ok(true);
}

/// Creates a bare-minimum config
pub(crate) fn create_empty_config() -> Config {
    let searching = vec![SearchInfo {
        search_type: "search".to_string(),
        available: true,
        supported_params: vec!["q".to_string()],
    }];

    let subcategories = vec![Subcategory {
        id: 1010,
        name: "b".to_string(),
    }];

    let categories = vec![Category {
        id: 1000,
        name: "a".to_string(),
        subcategories: subcategories,
    }];

    let genres = vec![Genre {
        id: 1,
        category_id: 1000,
        name: "c".to_string(),
    }];

    let tags = vec![Tag {
        name: "a".to_string(),
        description: "b".to_string(),
    }];

    return Config {
        search: dummy_search_func,
        auth: Some(dummy_auth_func),
        caps: Caps {
            server_info: Some(hashmap!(
                ("title".to_string(), "Test Torznab server".to_string()),
                ("email".to_string(), "test@example.com".to_string()),
                ("version".to_string(), "1.0".to_string())
            )),
            limits: Limits {
                max: 100,
                default: 20,
            },
            searching: searching,
            categories: categories,
            genres: Some(genres),
            tags: Some(tags),
        },
    };
}

#[cfg(test)]
mod tests {
    use crate::{api, dummy::create_empty_config, run};

    #[actix_rt::test]
    async fn api_with_empty_config() {
        run(create_empty_config()).await.unwrap();
    }

    #[actix_rt::test]
    async fn api_with_no_config() {
        // copied from lib.rs
        // in this case, CONFIG is still None
        // can't just use run() because that expects a Config, not an Option<Config>
        rocket::build()
            .mount("/", rocket::routes![api::caps, api::search])
            .launch()
            .await
            .unwrap();
    }
}
