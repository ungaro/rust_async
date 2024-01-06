use color_eyre::Report;
use futures::{stream::FuturesUnordered, StreamExt};
use std::future::Future;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{fmt, prelude::*};

use std::time::Duration;
use tokio::time::sleep;

use std::sync::Arc;
use tokio_rustls::{rustls::ClientConfig, TlsConnector};
use webpki::DNSNameRef;

use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

mod dumb;
mod tj;
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Hello from a comfy nest we've made for ourselves");

    pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
    pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";
    /*
        let res = tj::try_join(fetch_thing("first"), fetch_thing("second")).await?;
        info!(?res, "All done!");
    */

    info!("Joining...");
    let res = tj::try_join(fetch_thing("first"), fetch_thing("second")).await?;
    info!(?res, "All done!");

    /*
    let mut group = vec![
        fetch_thing(URL_1),
        fetch_thing(URL_2),
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();

    while let Some(item) = group.next().await {
        // propagate errors
        item?;
    }

    */
    /*
    let client = Client::new();

    let fut1 = fetch_thing(&client, URL_1);
    let fut2 = fetch_thing(&client, URL_2);

    fut1.await?;
    fut2.await?;
    -------------
        pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
        pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";


        let client = Client::new();
        fetch_thing(&client, URL_1).await?;
        fetch_thing(&client, URL_2).await?;
    */
    Ok(())
}

fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

async fn fetch_thing(name: &str) -> Result<&str, Report> {
    // look out it's port 443 now
    let addr: SocketAddr = ([1, 1, 1, 1], 443).into();
    let socket = TcpStream::connect(addr).await?;

    // establish a TLS session...
    let connector: TlsConnector = {
        let mut config = ClientConfig::new();
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        Arc::new(config).into()
    };
    // we have to use the proper DNS name now      👇
    let dnsname = DNSNameRef::try_from_ascii_str("one.one.one.one")?;
    let mut socket = connector.connect(dnsname, socket).await?;

    // we're writing straight to the socket, there's no buffering
    // so no need to flush
    socket.write_all(b"GET / HTTP/1.1\r\n").await?;
    //                        👇
    socket.write_all(b"Host: one.one.one.one\r\n").await?;
    socket.write_all(b"User-Agent: cool-bear\r\n").await?;
    socket.write_all(b"Connection: close\r\n").await?;
    socket.write_all(b"\r\n").await?;

    let mut response = String::with_capacity(256);
    socket.read_to_string(&mut response).await?;

    let status = response.lines().next().unwrap_or_default();
    info!(%status, %name, "Got response!");

    // dropping the socket will close the connection

    Ok(name)
}

/*
#[allow(clippy::manual_async_fn)]
fn fetch_thing<'a>(
    client: Client,
    url: &'static str,
) -> impl Future<Output = Result<(), Report>> + 'static {
    async move {
        let res = client.get(url).send().await?.error_for_status()?;
        info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
        Ok(())
    }
}
*/
/*
async fn fetch_thing(client: &Client, url: &str) -> Result<(), Report> {
    let res = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
    Ok(())
}
*/
fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    Ok(())
}
