pub(crate) type AuthFunc = fn(String) -> Result<String, String>;
pub(crate) type SearchFunc = fn(String, Vec<String>) -> Result<String, String>;

#[derive(Debug, Clone)]
pub struct ServerInfo {
    /// Specify the ServerInfo to be listed in <server> for `/api?t=caps`
    ///
    /// These fields are just those listed in the example on [torznab.github.io](https://torznab.github.io), there's no actual specification for thse fields.
    /// TODO: Update this to have customizable fields instead
    pub title: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Limits {
    /// The maximum and defaults for the `limit` parameter in queries
    /// `max` is the maximum number of results the program can return
    /// `default` is the default number of results the program will return
    /*
      I don't know why this would possibly need to be a u64, I can't imagine you'll be returning 18 quintillion results or whatever
      In fact, I *really* hope you aren't - if you are, you're doing something extremely wrong
      But hey, it's an option
    */
    pub max: u64,
    pub default: u64,
}

#[derive(Debug, Clone)]
pub struct SearchInfo {
    /// A struct holding the info for a type of search
    /// - `search_type` must be `search`, `tv-search`, `movie-search`, `audio-search`, or `book-search`
    /// - `available`
    pub search_type: String,
    pub available: bool,
    pub supported_params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Subcategory {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub subcategories: Vec<Subcategory>,
}

#[derive(Debug, Clone)]
pub struct Genre {
    pub id: String,
    pub category_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub category_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Caps {
    /// Holds the configuration for the capabilities of the Torznab server
    ///
    /// - server_info: `ServerInfo`
    ///   - see: `ServerInfo` docs
    /// - limits: `Limits`
    ///   - specifies the max and default items listed when searching
    ///   - see: `Limits` docs
    /// - searching: `Vec<SearchInfo>`
    ///   - specifies the capabilities of each search mode
    ///   - see: `SearchInfo` docs
    /// - categories: `Vec<Category>`
    ///   - lists known categories
    ///   - see: `Category` docs
    /// - genres: `Option<Vec<Genre>>`
    ///   - lists known genres, optional
    ///   - see: `Genre` docs
    ///
    /// <div class="warning">Note that this library might not support all the capabilities listed in yet, so check the README before listing capabilities, or just accept that unsupported capabilities will return error 404.
    ///
    /// It's recommended to add any capabilities you want, and set `available` to `false` in the `Caps` struct for any currently unsupported search types.</div>
    ///
    ///
    /// TODO: Add a way to partially(?) generate automatically from the Config
    pub server_info: ServerInfo,
    pub limits: Limits,
    pub searching: Vec<SearchInfo>,
    pub categories: Vec<Category>,
    pub genres: Option<Vec<Genre>>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone)]
pub struct Config {
    /// A struct that holds configuration for torznab-toolkit
    /// A search function (/api?t=search) and capabilities (/api?t=caps - struct Caps) required
    /// Everything else is optional
    pub search: SearchFunc, // NOTE: This is NOT optional,
    pub auth: Option<AuthFunc>,
    pub caps: Caps,
    pub tvsearch: Option<SearchFunc>,
    pub movie: Option<SearchFunc>,
    pub music: Option<SearchFunc>,
    pub book: Option<SearchFunc>,
}
