pub mod mp3s;
pub mod myfreemp3;

use crate::types::{Music, MythraResult};
use crate::utils::render_select_music;

use log::error;

pub async fn search_all(engine: &str, query: &str) -> MythraResult<Vec<Music>> {
    let query = String::from(query);
    match engine {
        "mp3s" => {
            let e = mp3s::MP3S {};
            e.search(query).await
        }
        "myfreemp3" => {
            let e = myfreemp3::MyFreeMP3 {};
            e.search(query).await
        }
        _ => {
            error!("Engine is unsupported");
            Err(Box::from("Exiting code"))
        }
    }
}

pub async fn cli(engine: &str, query: &str) {
    let title: &str = &(format!("Searching {} for {}", engine, query))[..];
    let results = search_all(engine, query).await.unwrap();
    render_select_music(results, title);
}