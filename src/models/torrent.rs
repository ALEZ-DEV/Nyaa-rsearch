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
