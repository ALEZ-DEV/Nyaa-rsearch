use std::error::Error;
use scraper::Html;
use crate::{build_url, get_all_torrent, get_max_pagination, SearchInput, SearchResult};

pub async fn search(search_input: SearchInput) -> Result<SearchResult, Box<dyn Error>> {
    let query = build_url(
        &search_input.search_input,
        &search_input.category_input,
        &search_input.page_input,
    );
    let response = reqwest::get(query).await?.text().await?;

    let document = Html::parse_document(&response);
    let max_pagination = get_max_pagination(&document)?;
    let all_torrent = get_all_torrent(&document)?;

    let result = SearchResult {
        search: search_input.search_input,
        category: search_input.category_input,
        current_page: search_input.page_input,
        page_max: max_pagination,
        torrents: all_torrent,
    };

    Ok(result)
}