use dotenv::dotenv;
use reqwest::Client;
use serde::Serialize;
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct DnsUpdateRequest {
    r#type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_token = env::var("API_TOKEN")?;
    let zone_id = env::var("ZONE_ID")?;
    let record_id = env::var("RECORD_ID")?;

    let public_ip = get_public_ip().await?;
    println!("Public IP: {}", public_ip);

    match update_dns_record(&api_token, &zone_id, &record_id, &public_ip).await {
        Ok(_) => {
            println!("DNS record updated successfully!");
        }
        Err(_) => {
            println!("Failed to update DNS record.");
        }
    }

    Ok(())
}

async fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get("https://api.ipify.org").send().await?;

    let ip = res.text().await?;
    Ok(ip)
}

async fn update_dns_record(
    api_token: &str,
    zone_id: &str,
    record_id: &str,
    ip_address: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id, record_id
    );

    let domain = env::var("DOMAIN")?;
    let ttl: u32 = env::var("TTL")?.parse()?;

    let dns_update = DnsUpdateRequest {
        r#type: "A".to_string(),
        name: domain,
        content: ip_address.to_string(),
        ttl: ttl,
        proxied: false,
    };

    let res = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&dns_update)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Error: {}", res.status()).into())
    }
}
