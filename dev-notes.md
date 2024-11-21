# Dev notes

## Resources

- <https://torznab.github.io/spec-1.3-draft/index.html>
- <https://www.git.je/Mirrors/Sonarr/wiki/Implementing-a-Torznab-indexer>
- for testing: <https://fosstorrents.com/thankyou/?name=debian&cat=Installation%20-%20amd64&id=0&hybrid=0>

---

```rs
struct TorznabToolkitConfig {
    auth_func: auth,
    search_func: search,
    whateverotherfunc: otherfunc,
    port: 5309
}

fn launch() -> Result {
    rocket::build().mount("/", routes![tt::search, tt:otherfunc])
}
```
