use std::fmt::format;

use super::categories::Categories;

#[derive(Debug)]
pub struct Torrent {
    pub name: String,
    pub category: Categories,
    pub torrent_file: String,
    pub magnet_link: String,
    pub size: String, //TODO: change the size of the file to a real number
    pub date: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub approved: i64,
}

impl Torrent {
    pub fn info(&self) -> String {
        let name = format!("name -> {}\n", self.name);
        let category = format!("category -> {:?}\n", self.category);
        let torrent_link = format!("torrent download link -> {}\n", self.torrent_file);
        let magnet_link = format!("magnet link -> {}\n", self.magnet_link);
        let size = format!("size -> {}\n", self.size);
        let date = format!("date (timestamp) -> {}\n", self.date);
        let seeders = format!("nbrs of seeders -> {}\n", self.seeders);
        let leechers = format!("nbrs of leechers -> {}\n", self.leechers);
        let approved = format!("nbrs of approved -> {}\n", self.approved);
        format!(
            "{name}{category}{torrent_link}{magnet_link}{size}{date}{seeders}{leechers}{approved}"
        )
    }
}
