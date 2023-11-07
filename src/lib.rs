use std::error::Error;

pub mod models;
use models::{categories::Categories, error::NyaaError, torrent::Torrent};
use scraper::{ElementRef, Html, Selector};

const URL: &str = "https://nyaa.si";

fn build_url(input: &str, category: &Categories, page: &i64) -> String {
    let research = format!("q={}", input);
    let category = format!("c={}", category.value());
    let page = format!("p={}", page);

    format!("{}/?{}&{}&{}", URL, research, category, page)
}

pub fn search(search_input: SearchInput) -> Result<SearchResult, Box<dyn Error>> {
    let query = build_url(
        &search_input.search_input,
        &search_input.category_input,
        &search_input.page_input,
    );
    let response = reqwest::blocking::get(query)?.text()?;

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

fn get_max_pagination(document: &Html) -> Result<i64, Box<dyn Error>> {
    let pagination_selector = Selector::parse(r#"ul[class="pagination"]"#)?;
    let li_selector = Selector::parse("li")?;
    let a_selector = Selector::parse("a")?;

    let ul = document.select(&pagination_selector).next().unwrap();

    let li_array: Vec<ElementRef> = ul.select(&li_selector).collect();
    Ok(li_array[li_array.len() - 2]
        .select(&a_selector)
        .next()
        .unwrap()
        .inner_html()
        .parse::<i64>()?)
}

fn get_all_torrent(document: &Html) -> Result<Vec<Torrent>, Box<dyn Error>> {
    let div_selector = Selector::parse(r#"div[class="table-responsive"]"#)?;
    let table_selector = Selector::parse("table")?;
    let tbody_selector = Selector::parse("tbody")?;
    let tr_selector = Selector::parse("tr")?;
    let td_selector = Selector::parse("td")?;
    let a_selector = Selector::parse("a")?;

    let mut result: Vec<Torrent> = vec![];

    let div = document.select(&div_selector).next().unwrap();
    let table = div.select(&table_selector).next().unwrap();
    let tbody = table.select(&tbody_selector).next().unwrap();
    for tr in tbody.select(&tr_selector) {
        let td_array: Vec<ElementRef> = tr.select(&td_selector).collect();

        let torrent_links: Vec<ElementRef> = td_array[2].select(&a_selector).collect();
        let a_name: Vec<ElementRef> = td_array[1].select(&a_selector).collect();
        let mut name_index = 0;
        if a_name.len() >= 2 {
            name_index = 1;
        }

        let torrent = Torrent {
            name: a_name[name_index]
                .value()
                .attr("title")
                .unwrap()
                .to_string(),
            category: Categories::parse(
                td_array[0]
                    .select(&a_selector)
                    .next()
                    .unwrap()
                    .value()
                    .attr("title")
                    .unwrap(),
            ),
            torrent_file: format!("{}{}", URL, torrent_links[0].value().attr("href").unwrap()),
            magnet_link: torrent_links[1].value().attr("href").unwrap().to_string(),
            size: td_array[3].inner_html(),
            date: td_array[4]
                .value()
                .attr("data-timestamp")
                .unwrap()
                .parse::<i64>()?,
            seeders: td_array[5].inner_html().parse::<i64>()?,
            leechers: td_array[6].inner_html().parse::<i64>()?,
            approved: td_array[7].inner_html().parse::<i64>()?,
        };

        result.push(torrent);
    }

    Ok(result)
}

#[derive(Debug)]
pub struct SearchInput {
    search_input: String,
    page_input: i64,
    category_input: Categories,
}

impl SearchInput {
    pub fn new(
        search_input: String,
        page_input: i64,
        category_input: Categories,
    ) -> Result<Self, NyaaError> {
        if page_input <= 0 {
            return Err(NyaaError::ImpossiblePagination);
        }

        Ok(SearchInput {
            search_input: search_input,
            page_input: page_input,
            category_input: category_input,
        })
    }
}

#[derive(Debug)]
pub struct SearchResult {
    search: String,
    category: Categories,
    current_page: i64,
    page_max: i64,
    torrents: Vec<Torrent>,
}

impl SearchResult {
    fn request(&mut self) -> Result<(), Box<dyn Error>> {
        let query = build_url(&self.search, &self.category, &self.current_page);

        let response = reqwest::blocking::get(query)?.text()?;

        let document = Html::parse_document(&response);
        let all_torrent = get_all_torrent(&document)?;

        self.torrents = all_torrent;

        Ok(())
    }

    pub fn info(&self) -> String {
        let search = format!("search -> {}\n", self.search);
        let category = format!("category -> {:?}\n", self.category);
        let page = format!("page -> {}\n", self.current_page);
        let max_page = format!("max page -> {}\n", self.page_max);
        let first_torrent = format!(
            "------first torrent------\n{}\n-------------------------\n",
            self.torrents[0].info()
        );
        let other_nbr_torrent = format!("{} torrent found in total\n", self.torrents.len());
        format!("{search}{category}{page}{max_page}{first_torrent}{other_nbr_torrent}")
    }

    pub fn next_page(&mut self) -> Result<(), Box<dyn Error>> {
        if self.current_page > self.page_max {
            return Err(Box::new(NyaaError::ImpossibleNext));
        }
        self.current_page += 1;
        self.request()?;

        Ok(())
    }

    pub fn previous_page(&mut self) -> Result<(), Box<dyn Error>> {
        if self.current_page - 1 < 1 {
            return Err(Box::new(NyaaError::ImpossiblePrevious));
        }
        self.current_page -= 1;
        self.request()?;

        Ok(())
    }
}
