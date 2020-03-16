extern crate serde_json;
extern crate unhtml;
#[macro_use]
extern crate unhtml_derive;

mod crawler;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::init();

    let mut step = 0;

    for url in config.urls.iter() {
        println!("Step #{}", step);
        let body = crawler::get_wiki_body(url).await?;
        crawler::run_crawler(body, &config.ignore.words, config.ignore.reverse);
        step += 1;
    }

    Ok(())
}