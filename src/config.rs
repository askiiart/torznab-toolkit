use crate::data::*;

type AuthFunc = fn(String) -> Result<String, String>;
type SearchFunc = fn(String, Vec<String>) -> Result<String, String>;

#[derive(Debug)]
pub struct Config {
    /// A struct that holds configuration for torznab-toolkit
    /// A search function (/api?t=search) and capabilities (/api?t=caps - struct Caps) required
    /// Everything else is optional
    pub search: SearchFunc,
    pub auth: Option<AuthFunc>,
    pub caps: Caps,
    pub tvsearch: Option<SearchFunc>,
    pub movie: Option<SearchFunc>,
    pub music: Option<SearchFunc>,
    pub book: Option<SearchFunc>,
}

#[derive(Debug)]
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
    /// TODO: Add a way to partially generate search capabilities automatically from the Config
    pub server_info: ServerInfo,
    pub limits: Limits,
    pub searching: Vec<SearchInfo>,
    pub categories: Vec<Category>,
    pub genres: Option<Vec<Genre>>,
    pub tags: Option<Vec<Tag>>,
}
