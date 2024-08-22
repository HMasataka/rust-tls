use reqwest::{Certificate, Client};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert_file = fs::read("credentials/develop/client.pem").expect("Cannot open CA file");
    let cert = Certificate::from_pem(&cert_file)?;

    let client = Client::builder()
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .use_rustls_tls()
        .build()
        .unwrap();

    let response = client.get("https://localhost:4443").send().await;

    match response {
        Ok(value) => println!("{:?}", value.content_length()),
        Err(e) => eprintln!("{:?}", e),
    }

    Ok(())
}
