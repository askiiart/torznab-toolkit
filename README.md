# Torznab Toolkit

A safe, multi-threaded, async toolkit for adding Torznab APIs to programs. You just focus on the indexer itself, we abstract away the hell that is the Torznab API.

Just fill in your own relevant functions and config ([`Config`]), and torznab-toolkit will run the API for you

```rust
use torznab_toolkit;
let config: torznab_toolkit::config::Config = /* config goes here */

torznab_toolkit::run(config);
```

To configure what it listens on, just change `ROCKET_ADDRESS` and `ROCKET_PORT`; see the [relevant docs](https://rocket.rs/guide/v0.5/deploying/) for details.

---

This program is brought to you by: metaphorical *and* literal truckloads of structs!

Note: I wrote the line above when I was tired. Don't ask me what *literal* truckloads of structs means, I don't know either.

## Functionality

| API call | Explanation                                                  | Implemented  |
| -------- | ------------------------------------------------------------ | -----------  |
| caps     | Returns the capabilities of the api.                         | ✅           |
| search   | Free text search query.                                      | ✅           |
| tvsearch | Search query with tv specific query params and filtering.    | ✅           |
| movie    | Search query with movie specific query params and filtering. | ✅           |
| music    | Search query with music specific query params and filtering. | ✅           |
| book     | Search query with book specific query params and filtering.  | ✅           |

<!-- for copy-pasting: ❌ ✅ -->
(copied from [torznab.github.io](https://torznab.github.io/spec-1.3-draft/torznab/Specification-v1.3.html))

## Limitations

- Currently this does not allow for returning errors from the program using the library, such as API limits for an account.
- Currently this does not allow for requiring authentication for `caps`; it's against spec (not that that's worth much), but common and perfectly fine to do.

## Notes

Thanks to [torznab.github.io](https://torznab.github.io/spec-1.3-draft/index.html), as it's my primary reference for this; NZBDrone's [Implementing a Torznab indexer](https://nzbdrone.readthedocs.io/Implementing-a-Torznab-indexer/) was also rather helpful.
