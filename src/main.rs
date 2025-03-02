use clap::{Arg, Command};
use std::error::Error;
use tokio::io::AsyncWriteExt; // For file writes

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Rust Downloader")
        .version("1.0")
        .about("Downloads files like curl")
        .arg(
            Arg::new("url")
                .help("The URL of the file to download")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("save_original")
                .help("Save the file with its original name (like curl -O)")
                .short('O')
                .long("save-original")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("save_to_file")
                .help("saves to a file with custom name")
                .short('o')
                .long("save-custom")
                .num_args(1)
                .help("specify filename")
                .value_name("FILENAME"),
        )
        .get_matches();

    // Retrieve URL and handle missing http(s) protocol
    let mut url = matches.get_one::<String>("url").unwrap().to_string();

    // Add http:// if not present
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url = format!("http://{}", url);
    }

    let save_original = matches.get_flag("save_original");
    let save_to_file = matches.contains_id("save_to_file");

    // Call the original scrap function to get the text content
    let content = scrap(&url).await?;

    // Determine the filename
    if save_original {
        let filename = url.rsplit('/').next().unwrap_or("downfile");
        save_response(&filename, &content).await?;
    } else if save_to_file {
        let filename = matches
            .get_one::<String>("save_to_file")
            .unwrap()
            .to_string();
        save_response(&filename, &content).await?;
    } else {
        println!("{}", content);
    }

    // Save the content to a file

    Ok(())
}

async fn scrap(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;

    Ok(response)
}

async fn save_response(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = tokio::fs::File::create(filename).await?;
    file.write_all(content.as_bytes()).await?;

    println!("File saved as {}", filename);
    Ok(())
}
