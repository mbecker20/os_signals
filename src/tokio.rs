use std::time::Duration;

use termination_signal::tokio::{term_signal_hook, ShutdownSignal};
use tokio::task::JoinHandle;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let (termination_handle, shutdown_signal) = term_signal_hook()?;

    tokio::select! {
        _ = termination_handle => {}
        _ = app(shutdown_signal) => {}
    }

    Ok(())
}

fn app(shutdown_signal: ShutdownSignal) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut count = 0;
        loop {
            if shutdown_signal.app_should_shutdown().await {
                let mut time_left = 3;
                while time_left > 0 {
                    println!("finishing in {time_left}");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    time_left -= 1;
                }
                shutdown_signal.app_finished_shutdown().await;
                break;
            } else {
                println!("count: {count}");
                count += 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    })
}
