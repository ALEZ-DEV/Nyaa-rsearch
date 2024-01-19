use std::error::Error;
use scraper::Html;
use crate::{build_url, get_all_torrent, get_max_pagination, is_page_empty, SearchInput, SearchResult};

///Will made a seach, if nothing is found will return a empty torrents list and max and min pagination will be set to 0
pub fn search(search_input: SearchInput) -> Result<SearchResult, Box<dyn Error>> {
    let query = build_url(
        &search_input.search_input,
        &search_input.category_input,
        &search_input.page_input,
    );
    let response = reqwest::blocking::get(query)?.text()?;

    let document = Html::parse_document(&response);

    if is_page_empty(&document) {
        return Ok(SearchResult {
            search: search_input.search_input,
            category: search_input.category_input,
            current_page: 0,
            page_max: 0,
            torrents: vec![],
        });
    }

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