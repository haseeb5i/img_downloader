extern crate reqwest;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_bytes = reqwest::get(
        "https://images.wallpaperscraft.com/image/sprout_leaves_branch_201627_1920x1080.jpg",
    )
    .await?
    .bytes()
    .await?;
    let mut file = File::create("img.jpg").expect("create failed");
    file.write_all(&img_bytes).expect("write failed");
    Ok(())
}
