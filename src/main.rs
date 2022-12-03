use torust::{SearchEngine, SearchOptions, L33T};

#[tokio::main]
async fn main() {
  // for testing

  let t1 = L33T::search_torrent(SearchOptions {
    category: torust::Category::TVShows,
    query: "peripheral".to_string(),
    page: 0,
  })
  .await
  .unwrap();

  dbg!(t1);

  let t2 =
    L33T::get_magnet("https://1337x.to/torrent/5428374/The-Peripheral-S01E01-WEB-x264-PHOENiX/")
      .await
      .unwrap();

  dbg!(t2);
}
