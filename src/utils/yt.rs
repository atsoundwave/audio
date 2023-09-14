use rusty_ytdl::search::{YouTube, SearchResult};

pub async fn get_yt_cover(artist: &str, title: &str) -> Option<String> {
    let yt_client = YouTube::new().unwrap();

    let result = yt_client
        .search(format!("{} {}", artist, title), None)
        .await
        .unwrap();

    let video = result.first().unwrap();

    match video {
        SearchResult::Video(video) => {
            return Some(video.thumbnails.first().unwrap().url.to_owned());
        }
        _ => {}
    }

    None
}
