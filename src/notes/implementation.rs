//! Notes regarding the implementation of Torznab
//!
//! - Because the behavior of `length` is ambiguous, torznab-toolkit just sets it to 0; the size is just specified by the `size` attribute
//!   - See [here](https://torznab.github.io/spec-1.3-draft/revisions/1.0-Torznab-Torrent-Support.html) for details
//! - Many indexers do not have the appropriate behavior according to the spec when `limit` is negative, and that behavior doesn't even make sense; instead, it follows the behavior of other indexers, and just ignores `limit` if it's negative.
//! - If a link isn't specified for a Torrent (`link` in `other_attributes`), it will fall back to the .torrent URL, then the magnet URI; i.e. you don't have to specify `link` if you don't have a webpage for the torrent.
