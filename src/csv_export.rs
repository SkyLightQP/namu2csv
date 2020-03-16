use std::error::Error;
use csv::Writer;
use std::fs::File;

pub fn save_csv(filename: String, vec: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = File::create(format!("{}.csv", filename))
        .expect("파일 생성 중 오류가 발생하였습니다.");
    let mut wtr = Writer::from_writer(file);

    for url in vec.iter() {
        wtr.write_record(&[url])?;
    }

    wtr.flush()?;

    Ok(())
}