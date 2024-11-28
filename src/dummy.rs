//! Some dummy stuff for testing the API
use crate::data::*;

fn dummy_search_func(_a: String, _b: Vec<String>) -> Result<String, String> {
    return Ok("hi".to_string());
}

fn dummy_auth_func(_a: String) -> Result<bool, String> {
    return Ok(true);
}

/// Creates a bare-minimum config
pub(crate) fn create_empty_config() -> Config {
    let mut searching = Vec::new();
    searching.push(SearchInfo {
        search_type: "search".to_string(),
        available: true,
        supported_params: vec!["id".to_string()],
    });

    let mut subcategories = Vec::new();
    subcategories.push(Subcategory {
        id: 1010,
        name: "b".to_string(),
    });

    let mut categories = Vec::new();
    categories.push(Category {
        id: 1000,
        name: "b".to_string(),
        subcategories: subcategories,
    });

    let mut genres = Vec::new();
    genres.push(Genre {
        id: 1,
        category_id: 1000,
        name: "c".to_string(),
    });

    let mut tags = Vec::new();
    tags.push(Tag {
        name: "a".to_string(),
        description: "b".to_string(),
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
            limits: Limits { max: 1, default: 1 },
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

#[cfg(test)]
mod tests {
    use crate::{api, dummy::create_empty_config, run};

    #[test]
    fn caps_test_with_empty_config() {
        unsafe {
            crate::api::CONFIG = Some(create_empty_config());
            println!("{:?}", crate::api::CONFIG);
        }
        println!("{:?}", crate::api::caps());
    }

    #[test]
    fn caps_test_no_config() {
        unsafe {
            println!("{:?}", crate::api::CONFIG);
        }
        println!("{:?}", crate::api::caps());
    }

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
