use crate::meme::{InfoMemeResponse, MemeResponse};
use reqwest::Error;

const RANDOM_MEME_API: &str = "https://meme-api.com/gimme";

pub async fn get_random_meme_information() -> Result<InfoMemeResponse, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(RANDOM_MEME_API)
        .header("accept", "application/json")
        .send()
        .await?;

    let meme_data = response.json::<InfoMemeResponse>().await?;

    Ok(meme_data)
}

pub async fn download_random_meme() -> Result<MemeResponse, Box<dyn std::error::Error>> {
    let meme_info = get_random_meme_information().await?;
    let meme_url = meme_info.preview.last().unwrap();

    let client = reqwest::Client::new();

    let response = client.get(meme_url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Error on download: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    Ok(MemeResponse {
        title: meme_info.title,
        content: bytes.to_vec(),
    })
}
