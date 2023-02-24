use std::fs;
use std::fs::File;
use std::io::prelude::*;
use md5;

const API_URL: &str = "https://api.thisissand.com/v2/pieces";    
const SALT_PHRASE: &str = "ranskanperunat";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let img_file = fs::read("img.jpg").unwrap();
    let digest = md5::compute(&img_file);

    let mut user_file = File::open("username.txt").unwrap();
    let mut username = String::new();
    user_file.read_to_string(&mut username)?;

    let mut caption_file = File::open("caption.txt").unwrap();
    let mut caption = String::new();
    caption_file.read_to_string(&mut caption)?;

    let salted_digest = md5::compute(format!("{}{}", SALT_PHRASE, caption));

    println!("fileHash: {:x} hash: {:x}", digest, salted_digest);

    let file_part = reqwest::multipart::Part::bytes(img_file)
        .file_name("blob")
        .mime_str("image/jpeg")
        .unwrap();

    let form = reqwest::multipart::Form::new()
        .part("file", file_part)
        .text("hash", format!("{:x}", salted_digest))
        .text("fileHash", format!("{:x}", digest))
        .text("username", username)
        .text("caption", caption)
        .text("mode", "web")
        .text("hd", "0")
        .text("appVersion", "2.5.0");
    let client = reqwest::Client::new();
    let result = match client
        .post(API_URL)
        .multipart(form)
        .send()
        .await
    {
        Ok(res) => res.text().await.unwrap_or("no message".to_string()),
        Err(_) => "{\"error\":400}".to_string(),
    };

    println!("{}", result);

    Ok(())
}
