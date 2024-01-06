use std::error::Error;
use std::time::Duration;

mod client;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    server::start();

    let client = client::Client::new()?;

    let req = client
        .request(reqwest::Method::GET, "http://localhost:1729")
        .build()?;
    let text = client.execute(req).await?.text().await?;
    log::info!("Request successful: {}", &text[..]);
    /*
    for _ in 0..5 {
        if let Err(e) = do_req().await {
            log::error!("{}", e);
            tokio::time::sleep(Duration::from_secs(1)).await;
        } else {
            break;
        }
    }
       */
    Ok(())
}

async fn do_req() -> Result<(), Box<dyn Error>> {
    let text = reqwest::get("http://localhost:1729")
        .await?
        .error_for_status()?
        .text()
        .await?;
    log::info!("Request successful: {}", &text[..]);
    Ok(())
}
