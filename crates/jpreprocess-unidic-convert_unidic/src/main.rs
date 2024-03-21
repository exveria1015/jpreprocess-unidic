use csv::{ReaderBuilder, WriterBuilder, StringRecord, Trim};
use jpreprocess_core::pronunciation::{Pronunciation, PronunciationParseError};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use regex::Regex;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: program_name input.csv".into());
    }
    let input_path = &args[1];
    let output_path = &args[2];

    // ログファイルを開く（存在しない場合は作成、存在する場合は追記）
    let mut log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("error_log.txt")?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .from_path(input_path)?;
    let mut wtr = WriterBuilder::new()
        .from_path(output_path)?;

    let re = Regex::new(r"\*/\d+")?;

    for result in rdr.records() {
        let record = result?;
        if record.len() != 33 {
            continue; // 33列ではないレコードはスキップ
        }

        let mora_size = match calculate_mora_size(&record[13]) {
            Ok(size) => size.to_string(),
            Err(e) => {
                writeln!(log_file, "Error in record {}: {:?}", record.position().unwrap().line(), e)?;
                "*".to_string() // エラーが発生した場合は"*"を使用
            },
        };

        let multiple_accents = record[28].contains(",");
        let accent_part = if multiple_accents {
            record[28].split(',').next().unwrap_or("")
        } else {
            &record[28]
        };

        let accent_rule = format!("{}/{}", accent_part, mora_size);
        let row_29_value = record[29].replace(",", "/");
        let replaced_accents = re.replace_all(&accent_rule, "*").to_string();

        let mut new_record = StringRecord::new();
        for &index in &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 14, 24, 13] {
            new_record.push_field(&record[index]);
        }
        new_record.push_field(&replaced_accents);
        new_record.push_field(&row_29_value);
        new_record.push_field(&record[30]);

        wtr.write_record(&new_record)?;
    }

    wtr.flush()?;
    Ok(())
}


// calculate_mora_size 関数を使用してモーラ数を計算
pub fn calculate_mora_size(read: &str) -> Result<usize, PronunciationParseError> {
    let pronunciation = Pronunciation::parse(read, 0)?;
    Ok(pronunciation.mora_size())
}
