#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct SearchInfo {
    /// A struct holding the info for a type of search
    /// - `search_type` must be `search`, `tv-search`, `movie-search`, `audio-search`, or `book-search`
    /// - `available`
    pub search_type: String,
    pub available: bool,
    pub supported_params: Vec<String>,
}

#[derive(Debug)]
pub struct Subcategory {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub subcategories: Vec<Subcategory>,
}

#[derive(Debug)]
pub struct Genre {
    pub id: String,
    pub category_id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Tag {
    pub id: String,
    pub category_id: String,
    pub name: String,
}
