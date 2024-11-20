use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.rfc-editor.org/rfc/rfc9655.txt").await?;

    let content = resp.text().await?;

    let formatted_content: String = content
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<&str>>()
        .join("\n");

    println!("{}", formatted_content);

    Ok(())
}
