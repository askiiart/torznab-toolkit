//! Some dummy stuff for testing the API
use crate::data::*;

fn dummy_search_func(a: String, b: Vec<String>) -> Result<String, String> {
    return Ok("hi".to_string());
}

fn dummy_auth_func(a: String) -> Result<bool, String> {
    return Ok(true);
}

/// Creates a bare-minimum config
pub fn create_empty_config() -> Config {
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

#[cfg(test)]
mod tests {
    use crate::{dummy::create_empty_config, run};

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
    async fn api_with_empty() {
        run(create_empty_config()).await.unwrap();
    }
}
