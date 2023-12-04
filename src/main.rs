use reqwest;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Deserialize)]
struct CTResponse {
    // Define the structure according to the JSON response of the API
    // This is a placeholder example
    entries: Vec<CertificateEntry>,
}

#[derive(Deserialize)]
struct CertificateEntry {
    // Adjust fields according to the API's response structure
    leaf_input: String,
}

async fn fetch_certificates(url: &str) -> Result<Vec<CertificateEntry>, reqwest::Error> {
    let resp = reqwest::get(url).await?.json::<CTResponse>().await?;
    Ok(resp.entries)
}

#[tokio::main]
async fn main() {
    const POLL_INTERVAL: Duration = Duration::from_secs(5); // Interval between API calls
    for i in 0..10 {
        let CT_API_URL: String = format!("https://ct.googleapis.com/logs/argon2021/ct/v1/get-entries?start={}&end={}", i, i+10);

        loop {
            match fetch_certificates(&CT_API_URL).await {
                Ok(certificates) => {
                    for cert in &certificates {
                        println!("New Certificate: {}", cert.leaf_input);
                    }
                }
                Err(e) => println!("Error fetching certificates: {}", e),
                }
        }
        sleep(POLL_INTERVAL).await;
    }
}
