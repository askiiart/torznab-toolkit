//! Contains tons of structs used by the library
//! 
//! All examples here are based off the [Torznab spec](https://torznab.github.io/spec-1.3-draft/torznab/Specification-v1.3.html)'s `/api?caps` example.
use std::collections::HashMap;
pub(crate) type AuthFunc = fn(String) -> Result<bool, String>;
pub(crate) type SearchFunc = fn(SearchParameters) -> Result<Vec<Torrent>, String>;

// TODO: Redo this all so that is uses builders with `AsRef<str>` arguments instead

#[derive(Debug, Clone, PartialEq, Eq)]
/// The maximum and defaults for the `limit` parameter in queries
/// `max` is the maximum number of results the program can return
/// `default` is the default number of results the program will return
///
/// Example:
/// ```
/// let query_limits = Limits {
///     max: 100, // maximum of 100 results per search query
///     default: 50,  // default of 50
/// };
/// ```
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
///
/// Example:
/// ```
/// let tv_query_search_info = SearchInfo {
///     search_type: "tv-search".to_string(),
///     available: true,
///     supported_params: vec!["q", "rid", "tvdbid", "season", "ep"]
///     .into_iter().map(|i| i.to_string()).collect::<String>(), // this bit's just to make all the `str`s to `String`s
/// };
/// ```
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
/// Contains subcategories, for use in [`Category`]
///
/// Example:
/// ```
/// let subcat = Subcategory {
///     id: 2010,
///     name: "Foreign".to_string(),
/// };
/// ```
pub struct Subcategory {
    /// The numeric ID of a subcategory
    ///
    /// The (de facto?) standard is `xxyy`, xx being the first two digits of the category, and the last two digits specifying the subcategory; see also: Category
    pub id: u32,
    /// The name of the subcategory, e.g. "Anime" under the "TV" cateogyr
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains a category, for use in [`Caps`] and searches as a query parameter
/// 
/// Example, using `subcat` from the [`Subcategory`] example:
/// ```
/// let category = Category {
///     id: 2000,
///     name: "Movies".to_string(),
///     subcategory: vec![subcat],
/// };
/// ```
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
/// Contains a genre, for use in [`Caps`] and searches as a query parameter
/// 
/// Example:
/// ```
/// let genre = Genre {
///     id: 1,
///     category_id: 5000,
///     name: "Kids".to_string(),
/// };
/// ```
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
/// Contains a tag, for use in [`Caps`] and searches as a query parameter
/// 
/// Example:
/// 
/// ```
/// let tag = Tag {
///     name: "trusted".to_string(),
///     description: "Uploader has high reputation".to_string(),
/// };
/// ```
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
/// Example, using other examples:
/// ```
/// let mut info: HashMap<String, String> = HashMap::new();
/// info.insert("version".to_string(), "1.1".to_string());
/// 
/// let caps_data = Caps {
///     server_info: Some(info),
///     limits: query_limits,
///     searching: vec![tv_query_search_info],
///     categories: vec![category],
///     genres: Some(vec![genre]),
///     tags: Some(vec![tag]),
/// };
/// ```
pub struct Caps {
    /// The server info, like title - optional
    ///
    /// Examples: `version`, `title`, `email`, `url`, `image`
    pub server_info: Option<HashMap<String, String>>,
    /// The max and default number of items to be returned by queries
    pub limits: Limits,
    /// Info about each type of search
    pub searching: Vec<SearchInfo>,
    /// What categories the server has
    pub categories: Vec<Category>,
    /// What genres the server has (optional)
    pub genres: Option<Vec<Genre>>,
    /// What torrents can be tagged with (optional)
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A struct that holds configuration for torznab-toolkit
/// The search function (`/api?t=search`) and capabilities (`/api?t=caps` - struct [`Caps`]) are required
/// Everything else is optional
/// 
/// Example, using other examples:
/// ```
/// fn search_func(parameters: SearchParameters) -> Result<Vec<Torrent>, String> {
///     let torrent = /* see `Torrent` example */
///     return torrent;
/// }
/// 
/// fn auth_func(apikey: String) -> Result<bool, String> {
///     if apikey == "letmein".to_string() {    
///         return Ok(true);
///     }
///     return Ok(false);
/// }
/// 
/// let conf = Config {
///     search: search,
///     auth: Some(auth),
///     caps: caps_data,
/// }
/// ```
pub struct Config {
    /// The function to use for all search types
    ///
    /// What search types are available is dependent on what's marked as available in the `searching` field of `caps` ([`Caps`])
    ///
    /// Search types: `search`, `tv-search`, `movie-search`, `audio-search`, `book-search`
    pub search: SearchFunc,
    /// The auth function - if not specified, then no authorization is needed.
    pub auth: Option<AuthFunc>,
    /// The capabilities of the indexer
    pub caps: Caps,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Holds the parameters for a search query
pub struct SearchParameters {
    /// What type of search this is
    ///
    /// Search types: `search`, `tv-search`, `movie-search`, `audio-search`, `book-search`
    pub search_type: String,
    /// The text query for the search
    pub q: Option<String>,
    /// The apikey, for authentication
    pub apikey: Option<String>,
    /// A [`Vec`] containing the numeric category IDs to be included in the search results
    pub categories: Option<Vec<u32>>,
    /// A [`Vec`] containing the extended attribute names to be included in the search results
    pub attributes: Option<Vec<String>>,
    /// Whether *all* extended attributes should be included in the search results; overrules `attributes`
    pub extended_attrs: Option<bool>,
    /// How many items to skip/offset by in the results.
    pub offset: Option<u32>,
    /// The maximum number of items to return - also limited to whatever `limits` is in [`Caps`]
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Holds the info for a torrent
///
/// Any attributes not listed here are optional, and can be put in `other_attributes`; **however**, the following are recommended:
/// - `seeders`
/// - `leechers`
/// - `peers`
/// - `infohash`
/// - `link` (link to a webpage; if not specified, will fallback to `torrent_file_url`, then `magnet_uri`)
///
/// <div class="warning">One of either `torrent_file_url` or `magnet_uri` are required.</div>
/// Example:
/// ```
/// let torrent = Torrent {
///     title: "totally normal torrent".to_string(),
///     description: None,
///     size: 2484345508,
///     category_ids: vec![1010],
///     torrent_file_url: Some("http://localhost/totally-normal.torrent".to_string()),
///     magnet_uri: Some("magnet:?xt=urn:btih:blahblahblahdothechachacha".to_string()),
///     other_attributes: None,
/// };
/// ```
pub struct Torrent {
    /// The title of the torrent
    pub title: String,
    /// The description of the torrent - optional
    pub description: Option<String>,
    /// The size of the torrent, **in bytes**
    pub size: u64,
    /// A vector of (sub)category IDs
    pub category_ids: Vec<u32>,
    /// The URL of the `.torrent` file
    pub torrent_file_url: Option<String>,
    /// The magnet URI o the torrent
    pub magnet_uri: Option<String>,
    /// Any other attributes
    pub other_attributes: Option<HashMap<String, String>>,
}
