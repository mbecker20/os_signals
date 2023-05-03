use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use termination_signal::sync::{term_signal_hook, ShutdownSignal};

pub fn main() -> anyhow::Result<()> {
    let (termination_handle, shutdown_signal) = term_signal_hook()?;

    let app = app(shutdown_signal);

    loop {
        if app.is_finished() {
            break;
        }
        if termination_handle.is_finished() {
            break;
        }
        sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn app(shutdown_signal: ShutdownSignal) -> std::thread::JoinHandle<()> {
    spawn(move || {
        let mut count = 0;
        loop {
            if shutdown_signal.app_should_shutdown() {
                let mut time_left = 3;
                while time_left > 0 {
                    println!("finishing in {time_left}");
                    sleep(Duration::from_secs(1));
                    time_left -= 1;
                }
                shutdown_signal.app_finished_shutdown();
                break;
            } else {
                println!("count: {count}");
                count += 1;
                sleep(Duration::from_secs(1));
            }
        }
    })
}
