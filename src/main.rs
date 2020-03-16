extern crate csv;
extern crate serde_json;
extern crate unhtml;
#[macro_use]
extern crate unhtml_derive;

mod config;
mod crawler;
mod csv_export;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::init();

    let mut step = 0;

    for url in config.urls.iter() {
        println!("Step #{}", step);

        let body = crawler::get_wiki_body(url).await?;
        let data = crawler::run_crawler(body, &config.ignore.words, config.ignore.reverse)?;

        csv_export::save_csv(format!("result{}", step), data).expect("csv 변환에 실패했습니다.");

        step += 1;
    }

    Ok(())
}
