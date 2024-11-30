# Dev notes

## Resources

- <https://torznab.github.io/spec-1.3-draft/index.html>
- <https://www.git.je/Mirrors/Sonarr/wiki/Implementing-a-Torznab-indexer>
- <https://nzbdrone.readthedocs.io/Implementing-a-Torznab-indexer/>
- for testing: <https://fosstorrents.com/thankyou/?name=debian&cat=Installation%20-%20amd64&id=0&hybrid=0>

---

example usage:

```rust
let config = /* config goes here */

fn main() -> Result {
    run(config);
}
```

## Torznab spec

### Query results

Queries are returned as an RSS feed something like this:

```rusts
<rss version="1.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:torznab="http://torznab.com/schemas/2015/feed">
  <channel>
    <atom:link rel="self" type="application/rss+xml" />
    <title>Prowlarr</title>
    <item>
      <title>Item title</title>
      <description />
      <link>http://localhost:9999/</link>
      <torznab:attr name="category" value="1010" />
      <torznab:attr name="category" value="84570" />
      <torznab:attr name="size" value="1073741824" />
      <torznab:attr name="">
      <enclosure url="http://localhost/" length="0" type="application/x-bittorrent" />
    </item>
  </channel>
</rss>
```

Item attributes:

- RSS:
  - `title`: title of the thing; can maybe be empty? unsure - torznab-toolkit will treat this as required, and set description to be empty
  - `description`: description; can just be empty
  - `link`: a link
    - Preferably specified, if not then fallback to .torrent link, then magnet url
  - at least title *or* description is required; link is recommended for compatibility with some non-compliant software, like headphones (see [bitmagnet issue 349](https://github.com/bitmagnet-io/bitmagnet/issues/349))
- Torznab
  - Required:
    - `size`: Size in bytes
    - `category`: (Sub)category id; can occur multiple times for several categories
  - Recommended (by me, based off my own thoughts and NZBDrone's [Implementing a Torznab indexer](https://nzbdrone.readthedocs.io/Implementing-a-Torznab-indexer/) page):
    - `seeders`: Number of seeders
    - `leechers`: Number of leechers
    - `peers`: Number of peers
    - `infohash`: Torrent infohash
    - `magneturl`: The magnet URI for the torrent
  - URLs:
    - Main URI can either be a magnet URI or a link to a .torrent file: `<enclosure url="http://localhost/" length="0" type="application/x-bittorrent" />`
      - Length is ambiguous, so it will just be 0 (see below)
    - If .torrent URL is provided, use that, if not use the magnet; also put the magnet in `magneturl`
  - Rest of available attributes: <https://torznab.github.io/spec-1.3-draft/torznab/Specification-v1.3.html?highlight=server#predefined-attributes>

---

## Non-compliance and errors in the Torznab spec

- Prowlarr's built-in indexer for sites it scrapes (like 1337x) does not respect `limit`
- Negative `limit`: The spec says that if `limit` is negative, then to return `201 - Incorrect parameter`; but 201 is Created, and there's no Incorrect Parameter HTTP status code (my best guess is it should be 400?). Additionally, other indexers just ignore the limit if it's negative, so in that case, torznab-toolkit will just set the limit to the maximum to match the behavior of other widely-used indexers.

## Ambiguous behavior

- Due to ambiguous behavior, torznab-toolkit will set `length` to 0

> The correct definition of the enclosure length attribute should have been the size of the .torrent file, however newznab defined it as the length of the usenet download rather than the .nzb size.
> Therefore the length attribute can be either 0, or the .torrent/.nzb file size, or the actual media size. Given this ambiguity, services should instead provide the size extended attribute with the size of the media. (eg. <torznab:attr name=”size” value=”1460985071” />)

(from [Torznab spec - Torznab Torrent Support](https://torznab.github.io/spec-1.3-draft/revisions/1.0-Torznab-Torrent-Support.html]))
