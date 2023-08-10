# Nyaa-rsearch

***

A rust library for search Torrent on nyaa.si

## How to use it

***

### Simple research
```rust
use std::error::Error; 
use nyaa_rsearch::{models::categories, search}; // import required function

pub fn main() -> Result<(), Box<dyn Error>> {
    //research by name and category
    let search_result = search("Houseki No Kuni", categories::Categories::Anime).unwrap();
    //Display the result of the research
    println!("{}", search_result.info());

    Ok(())
}
```
### Result
```
search -> Houseki No Kuni
category -> Anime
page -> 1
max page -> 3
------first torrent------
name -> [Cleo] Houseki no Kuni | Land of the Lustrous [Dual Audio 10bit BD1080p][HEVC-x265]
category -> AnimeEnglishTranslated
torrent download link -> https://nyaa.si/download/1546687.torrent
magnet link -> magnet:?xt=urn:btih:cd2ea8ede11332d2a1a6a4bc6058fc73cb3ffe08&dn=%5BCleo%5D%20Houseki%20no%20Kuni%20%7C%20Land%20of%20the%20Lustrous%20%5BDual%20Audio%2010bit%20BD1080p%5D%5BHEVC-x265%5D&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce
size -> 4.5 GiB
date (timestamp) -> 1656314786
nbrs of seeders -> 79
nbrs of leechers -> 3
nbrs of approved -> 2807

-------------------------
75 torrent found in total
```

***

### Change page
```rust
use std::error::Error; 
use nyaa_rsearch::{models::categories, search}; // import required function

pub fn main() -> Result<(), Box<dyn Error>> {
    //research by name and category
    let mut search_result = search("Houseki No Kuni", categories::Categories::Anime).unwrap();
    search_result.next_page()?; // go to next page
    // search_result.previous_page()? // go to previous page
    println!("{}", search_result.info()); //Display the result of the research

    Ok(())
}

```
### Result
```
search -> Houseki No Kuni
category -> Anime
page -> 2
max page -> 3
------first torrent------
name -> [Nekomoe kissaten] Houseki no Kuni - 11 [WebRip 1920x1080 HEVC-yuv420p10 EAC3]
category -> AnimeRaw
torrent download link -> https://nyaa.si/download/989254.torrent
magnet link -> magnet:?xt=urn:btih:49993ee15382d31a1edbb98d6257ea20144759cf&dn=%5BNekomoe%20kissaten%5D%20Houseki%20no%20Kuni%20-%2011%20%5BWebRip%201920x1080%20HEVC-yuv420p10%20EAC3%5D&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce
size -> 521.1 MiB
date (timestamp) -> 1513736489
nbrs of seeders -> 0
nbrs of leechers -> 0
nbrs of approved -> 220

-------------------------
75 torrent found in total
```

***

## Installation

To install this library simply do 
```
cargo add nyaa-rsearch
```

## Contribute

Any improvement or issue is welcome for help this library to improve on it stability