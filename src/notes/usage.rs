//! Notes regarding the usage of torznab-tooolkit
//!
//! ---
//!
//! - Please implement the `season`, `ep`, and `id` attributes for torrents when possible
//!   - Implementing `id`, at least, is far out of scope of this library, and providing `season` and `ep` more effective than this library parsing for them. However, parsing for those as an optional fallback may be added later.
//!   - See [here](https://torznab.github.io/spec-1.3-draft/revisions/1.0-Torznab-Torrent-Support.html) for details
//!
//! TODO: Add better docs for using the library

// TODO: Add parsing for `season` and `ep`
