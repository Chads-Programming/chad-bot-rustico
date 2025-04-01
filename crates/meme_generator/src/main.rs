use meme_generator::api::get_random_meme_information;

#[tokio::main]
async fn main() {
    let meme = get_random_meme_information().await;

    println!("Meme {:?}", meme);
}
