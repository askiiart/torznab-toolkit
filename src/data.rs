//! Contains tons of structs used by the library
use rocket::FromForm;

pub(crate) type AuthFunc = fn(String) -> Result<bool, String>;
// TODO: Figure out what the arguments should be for a search function and what it should return
pub(crate) type SearchFunc = fn(String, Vec<String>) -> Result<String, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Specify the ServerInfo to be listed in <server> for `/api?t=caps`
///
/// These fields are just those listed in the example on [torznab.github.io](https://torznab.github.io), there's no actual specification for thse fields.
/// TODO: Update this to have customizable fields instead
pub struct ServerInfo {
    /// The title of the server
    pub title: Option<String>,
    /// The email for the server info
    pub email: Option<String>,
    /// The URL to the server's image (e.g. logo)
    pub image: Option<String>,
    /// What version the server is - unrelated to torznab-toolkit's version, but may be used by the program
    pub version: Option<String>,
}

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
/// <div class="warning">Note that this library might not support all the capabilities listed in yet, so check the README before listing capabilities, or just accept that unsupported capabilities will return error 404.
///
/// It's recommended to add any capabilities you want, and set `available` to `false` in the [`Caps`] struct for any currently unsupported search types.</div>
///
/// TODO: Add a way to partially(?) generate automatically from the Config
pub struct Caps {
    pub server_info: ServerInfo,
    pub limits: Limits,
    pub searching: Vec<SearchInfo>,
    pub categories: Vec<Category>,
    pub genres: Option<Vec<Genre>>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct that holds configuration for torznab-toolkit
/// The search function (`/api?t=search`) and capabilities (`/api?t=caps` - struct [`Caps`]) are required
/// Everything else is optional
pub struct Config {
    pub search: SearchFunc,
    pub auth: Option<AuthFunc>,
    pub caps: Caps,
    pub tvsearch: Option<SearchFunc>,
    pub movie: Option<SearchFunc>,
    pub music: Option<SearchFunc>,
    pub book: Option<SearchFunc>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromForm)]
/// A struct used by the API's search functions to hold its query parameters
/// Currently required (AFAIK) because of limitations with rocket
pub(crate) struct SearchForm {
    /// The text query for the search
    pub(crate) q: Option<String>,
    /// The apikey, for authentication
    pub(crate) apikey: Option<String>,
    /// The list of numeric category IDs to be included in the search results
    /// Returned by Rocket.rs as a string of comma-separated values, then split in the function to a `Vec<u32>`
    pub(crate) cat: Option<String>,
    /// The list of extended attribute names to be included in the search results
    /// Returned by Rocket.rs as a string of comma-separated values, then split in the function to a `Vec<String>`
    pub(crate) attrs: Option<String>,
    /// Whether *all* extended attributes should be included in the search results; overrules `attrs`
    /// Can be 0 or 1
    pub(crate) extended: Option<u8>,
    /// How many items to skip/offset by in the results.
    pub(crate) offset: Option<u32>,
    /// The maximum number of items to return - also limited to whatever `limits` is in [`Caps`]
    pub(crate) limit: Option<u32>,
}
