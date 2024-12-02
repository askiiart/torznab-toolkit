//! Notes regarding the usage of torznab-tooolkit, and its implementation of Torznab
//!
//! - Please implement whatever attributes for torrents you can whenever possible to improve search results; particularly, `season`, `ep`, and `*id` fields are recommended, besides those listed in [``]
//!   - Potential attributes are listed [here](https://torznab.github.io/spec-1.3-draft/torznab/Specification-v1.3.html#predefined-attributes)
//! - Because the behavior of `length` is ambiguous, torznab-toolkit just sets it to 0; the size is just specified by the `size` attribute
//!   - See [here](https://torznab.github.io/spec-1.3-draft/revisions/1.0-Torznab-Torrent-Support.html) for details
//! - Many indexers do not have the appropriate behavior according to the spec when `limit` is negative, and that behavior doesn't even make sense; instead, it follows the behavior of other indexers, and just ignores `limit` if it's negative.
//! - If a link isn't specified for a [`Torrent`] (`link` in `other_attributes` field), it will fall back to the .torrent URL, then the magnet URI; i.e. you don't have to specify `link` if you don't have a webpage for the torrent.
//!   - Regardless of this, `link` is optional, but some software (e.g. Headphones) breaks if it's not provided.
//! - Currently if a function returns an [`Err`], torznab-toolkit won't handle it and will just return 500; however, Rocket will log it to the console.

// imports for docs
use crate::data::*;
