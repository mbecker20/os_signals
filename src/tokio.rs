use std::time::Duration;

use futures::stream::StreamExt;
use signal_hook::consts::{SIGINT, SIGQUIT, SIGTERM};
use signal_hook_tokio::Signals;
use tokio::task::JoinHandle;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let term_signal = term_signal_hook()?;

    let app = tokio::spawn(async move {
        let mut count = 0;
        loop {
            println!("count: {count}");
            count += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::select! {
        _ = app => {}
        _ = term_signal => {}
    }

    Ok(())
}

fn term_signal_hook() -> anyhow::Result<JoinHandle<()>> {
    let mut signals = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])?;
    let handle = tokio::spawn(async move {
        while let Some(signal) = signals.next().await {
            println!("recv terminate signal: {signal}");
            break;
        }
    });
    Ok(handle)
}
