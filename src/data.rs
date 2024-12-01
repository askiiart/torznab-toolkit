//! Contains tons of structs used by the library

use std::collections::HashMap;

use rocket::FromForm;
pub(crate) type AuthFunc = fn(String) -> Result<bool, String>;
// TODO: Figure out what the arguments should be for a search function and what it should return
pub(crate) type SearchFunc = fn(String, Vec<String>) -> Result<String, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// The maximum and defaults for the `limit` parameter in queries
/// `max` is the maximum number of results the program can return
/// `default` is the default number of results the program will return
pub struct Limits {
    /*
      I don't know why this would possibly need to be a u32, I can't imagine you'll be returning 4 billion results or whatever
      In fact, I *really* hope you aren't - if you are, you're doing something extremely wrong
      But hey, it's an option
    */
    /// The maximum number of entries that can be listed in a search query
    pub max: u32,
    /// The default number of entries to be listed in a search query
    pub default: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct holding the info for a type of search
pub struct SearchInfo {
    /// What type of search this is - must be `search`, `tv-search`, `movie-search`, `audio-search`, or `book-search`
    pub search_type: String,
    /// Whether this search type is available
    pub available: bool,
    /// The supported parameters for this search type
    ///
    /// Highly recommended: `q` (free text query)
    pub supported_params: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains subcategories, for use in `Category`
pub struct Subcategory {
    /// The numeric ID of a subcategory
    ///
    /// The (de facto?) standard is `xxyy`, xx being the first two digits of the category, and the last two digits specifying the subcategory; see also: Category
    pub id: u32,
    /// The name of the subcategory, e.g. "Anime" under the "TV" cateogyr
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains a category, for use in `Caps` and searches as a query parameter
pub struct Category {
    /// The numeric ID of a category
    ///
    /// The (de facto?) standard is `xxyy`, xx being the first two digits of the category, and the last two digits specifying the subcategory; see also: Subcategory
    pub id: u32,
    /// The name of the category, e.g. "Movies"
    pub name: String,
    /// A vector of all the subcategory in this category
    pub subcategories: Vec<Subcategory>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains a genre, for use in `Caps` and searches as a query parameter
pub struct Genre {
    /// The numeric ID of a genre
    ///
    /// I'm not aware of any standard for numbering this; the specification for Torznab shows an example with an ID of 1.
    pub id: u32,
    /// The numeric ID of the category this genre is for.
    pub category_id: u32,
    /// The name of the genre
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains a tag, for use in `Caps` and searches as a query parameter
pub struct Tag {
    /// The name of a tag for a torrent
    pub name: String,
    /// The description of the tag
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Holds the configuration for the capabilities of the Torznab server (used in `/api?t=caps`)
///
/// <div class="warning">Note that this library might not support all the capabilities listed in yet, so check the README before listing capabilities, or just accept that unsupported capabilities will return error 501.
///
/// It's recommended to add any capabilities you want, and set `available` to `false` in the [`Caps`] struct for any currently unsupported search types.</div>
///
/// TODO: Add a way to partially(?) generate automatically from the Config
pub struct Caps {
    /// The server info, like title - optional
    ///
    /// Examples: `version`, `title`, `email`, `url`, `image`
    pub server_info: Option<HashMap<String, String>>,
    /// The max and default number of items to be returned by queries - see [`Limits`]
    pub limits: Limits,
    /// Info about each type of search
    pub searching: Vec<SearchInfo>,
    /// What categories the server has - see [`Category`]
    pub categories: Vec<Category>,
    /// What genres the server has - see [`Genre`] (optional)
    pub genres: Option<Vec<Genre>>,
    /// What torrents can be tagged with - see [`Tag`] (optional)
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct that holds configuration for torznab-toolkit
/// The search function (`/api?t=search`) and capabilities (`/api?t=caps` - struct [`Caps`]) are required
/// Everything else is optional
pub struct Config {
    /// The function to use for all search types
    ///
    /// What search types are available is dependent on what's marked as available in the `searching` field of `caps` ([`Caps`])
    ///
    /// Search types: `search`, `tv-search`, `movie-search`, `audio-search`, `book-search`
    pub search: SearchFunc,
    /// The auth function - if not specified, then no authorization is needed.
    pub auth: Option<AuthFunc>,
    /// The capabilities of the indexer - see [`Caps`]
    pub caps: Caps,
}

/// An internal struct to hold the parameters for a search
///
/// Before being sent to the "client" program, it's turned into a SearchParameters object by `to_search_param()`, adding the search type
#[derive(Debug, Clone, PartialEq, Eq, FromForm)]
pub(crate) struct InternalSearchParameters {
    /// The text query for the search
    pub(crate) q: Option<String>,
    /// The apikey, for authentication
    pub(crate) apikey: Option<String>,
    /// A [`Vec`] containing the numeric category IDs to be included in the search results
    pub(crate) categories: Option<Vec<u32>>,
    /// A [`Vec`] containing the extended attribute names to be included in the search results
    pub(crate) attributes: Option<Vec<String>>,
    /// Whether *all* extended attributes should be included in the search results; overrules `attributes`
    pub(crate) extended_attrs: Option<bool>,
    /// How many items to skip/offset by in the results.
    pub(crate) offset: Option<u32>,
    /// The maximum number of items to return - also limited to whatever `limits` is in [`Caps`]
    pub(crate) limit: u32,
}

impl InternalSearchParameters {
    pub(crate) fn to_search_param(&self, search_type: String) -> SearchParameters {
        return SearchParameters {
            search_type: search_type,
            q: self.q.clone(),
            apikey: self.apikey.clone(),
            categories: self.categories.clone(),
            attributes: self.attributes.clone(),
            extended_attrs: self.extended_attrs,
            offset: self.offset,
            limit: self.limit,
        };
    }
}

/// Holds the parameters for a search query
pub struct SearchParameters {
    /// What type of search this is
    ///
    /// Search types: `search`, `tv-search`, `movie-search`, `audio-search`, `book-search`
    pub(crate) search_type: String,
    /// The text query for the search
    pub(crate) q: Option<String>,
    /// The apikey, for authentication
    pub(crate) apikey: Option<String>,
    /// A [`Vec`] containing the numeric category IDs to be included in the search results
    pub(crate) categories: Option<Vec<u32>>,
    /// A [`Vec`] containing the extended attribute names to be included in the search results
    pub(crate) attributes: Option<Vec<String>>,
    /// Whether *all* extended attributes should be included in the search results; overrules `attributes`
    pub(crate) extended_attrs: Option<bool>,
    /// How many items to skip/offset by in the results.
    pub(crate) offset: Option<u32>,
    /// The maximum number of items to return - also limited to whatever `limits` is in [`Caps`]
    pub(crate) limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Holds the info for a torrent
pub struct Torrent {
    title: String,
    description: Option<String>,
    size: u64,
    categories: Vec<Category>,
    seeders: Option<u32>,
    leechers: Option<u32>,
    peers: Option<u32>,
    infohash: Option<String>,
    link: Option<String>,
    torrent_file_url: Option<String>,
    magnet_uri: Option<String>,
    other_attributes: Option<HashMap<String, String>>,
}
