#[warn(dead_code)]
pub async fn post(url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .send()
        .await?;
    Ok(response.text().await?)
}
