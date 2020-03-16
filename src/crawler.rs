use reqwest::Client;
use unhtml::{Error, FromHtml};

#[derive(Debug)]
#[derive(FromHtml)]
#[html(selector = ".wiki-heading")]
struct KartRiderTrack {
    #[html(selector = "span", attr = "inner")]
    track_name: Vec<String>
}

pub async fn get_wiki_body(url: &str) -> reqwest::Result<String> {
    let client = Client::builder()
        .user_agent("namu2csv-crawler")
        .build()?;
    let body = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}

pub fn run_crawler(body: String, ignore_words: &Vec<String>, is_ignore_reverse: bool) {
    let track: Result<KartRiderTrack, Error> = KartRiderTrack::from_html(&body);

    let mut count = 0;

    match track {
        Ok(track) => {
            track.track_name.iter().for_each(|item| {
                let item = item.replace("[편집]", "");

                if item == "" || ignore_words.contains(&item) {
                    return;
                }

                if is_ignore_reverse && item.contains("[리버스]") {
                    return;
                }

                println!("Map #{}: {}", count, item);

                count += 1;
            });
        }
        Err(e) => {
            println!("crawling error: {}", e);
        }
    }
}