use async_trait::async_trait;
use regex::Regex;
use std::{error::Error, fmt::Display};

// TODO: add other search engines
// TorrentGalaxy https://torrentgalaxy.to/
// Rarbg         https://rarbg2021.org/
// ThePirateBay  https://thepiratebay.org/

pub enum Category {
  Movies,
  TVShows,
}

pub struct SearchOptions {
  pub query: String,
  pub category: Category,
  pub page: u32, // >= 0
}

#[async_trait]
pub trait SearchEngine {
  async fn search_torrent(search: SearchOptions) -> Result<Vec<Torrent>, TorustError>;
  async fn get_magnet(torrent_link: &str) -> Result<String, TorustError>;
}

async fn get_page_text(url: &str) -> Result<String, TorustError> {
  Ok(reqwest::get(url).await?.text().await?)
}

pub struct L33T;

#[async_trait]
impl SearchEngine for L33T {
  async fn search_torrent(search: SearchOptions) -> Result<Vec<Torrent>, TorustError> {
    if search.query.len() < 3 {
      return Err(TorustError::SearchTooShort);
    }

    let encoded_query = urlencoding::encode(&search.query).to_string();

    let encoded_category = match &search.category {
      Category::Movies => "Movies",
      Category::TVShows => "TV",
    };

    let url = format!(
      "https://1337x.to/category-search/{}/{}/{}/",
      encoded_query,
      encoded_category,
      search.page + 1
    );

    let page = get_page_text(&url).await?;

    // println!("{}", page);

    let re = Regex::new(
      r#"<tr>\s*<td[\s\S]+?.*href="(/torrent.+?)">(.+)</a>[\s\S]+?coll-2.+?>(.+)</td>[\s\S]+?coll-3.+?>(.+)</td>[\s\S]+?coll-date.+?>(.+)</td>[\s\S]+?coll-4.+?>(.+)<span[\s\S]+?coll-5.+?><a href=".+">(.+)</a>"#,
    ).unwrap();

    let torrents = re
      .captures_iter(&page)
      .map(|cap| Torrent {
        torrent_link: format!("https://1337x.to{}", &cap[1]),
        title: cap[2].to_string(),
        seeders: cap[3].parse::<u32>().unwrap(),
        leechers: cap[4].parse::<u32>().unwrap(),
        uploaded: cap[5].to_string(),
        size: cap[6].to_string(),
        uploader: cap[7].to_string(),
      })
      .collect::<Vec<Torrent>>();

    Ok(torrents)
  }

  async fn get_magnet(torrent_link: &str) -> Result<String, TorustError> {
    let page = get_page_text(torrent_link).await?;

    // println!("{}", page);

    let re = Regex::new(r#""(magnet.+?)""#).unwrap();

    let magnet = &re.captures_iter(&page).next().unwrap()[1];
    let decoded_magnet = urlencoding::decode(magnet).unwrap().to_string();

    Ok(decoded_magnet)
  }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Torrent {
  pub torrent_link: String, // unique
  pub title: String,
  pub seeders: u32,
  pub leechers: u32,
  pub uploaded: String, // TODO: use https://docs.rs/chrono/latest/chrono/
  pub size: String, // TODO: use https://github.com/kennytm/parse-size or https://lib.rs/crates/bytesize
  pub uploader: String,
}

#[derive(Debug)]
pub enum TorustError {
  RequestError(String),
  SearchTooShort,
  // TODO: add more errors
}

impl Display for TorustError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use TorustError::*;
    match self {
      RequestError(e) => write!(f, "Request error: {}", e),
      SearchTooShort => write!(f, "Search was too short. Try adding more characters"),
    }
  }
}

impl Error for TorustError {}

impl From<reqwest::Error> for TorustError {
  fn from(e: reqwest::Error) -> Self {
    TorustError::RequestError(e.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    // TODO
  }
}
