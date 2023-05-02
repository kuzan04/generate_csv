use std::io::{Result, Write};
use std::fs::OpenOptions;
use uuid::Uuid;
use sha256::digest;
use base64::{Engine as _, engine::general_purpose};
use chrono::Local;

fn generate_data() -> Result<String> {
    let uuid = Uuid::new_v4().to_string();
    let hasher = digest(uuid.clone());
    let base64 = general_purpose::STANDARD.encode(uuid.clone());
    Ok(format!("{},{},{}", uuid, hasher, base64))
}

fn create_record(d: String, content: String) -> Result<()> {
    let now = Local::now().format("%Y-%m-%dT%H.00.00").to_string();
    let filename = format!("{}_{}.csv", d, now);
    let mut writer = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)
        .unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    // set headers
    if writer.metadata().unwrap().len() == 0 {
        writeln!(writer, "UUID,Hasher,Base64")?;
    }
    // record data
    writeln!(writer, "{}", content)?;
    Ok(())
}

pub fn dump(dir: String) {
    let host = hostname::get().unwrap_or_else(|_| String::from("unknow").into()).into_string().unwrap();
    loop {
        let data = generate_data().unwrap();
        create_record(format!("{}{}", dir, host), data).unwrap();
    }
}
