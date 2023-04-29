use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use signal_hook::{
    consts::{SIGINT, SIGQUIT, SIGTERM},
    iterator::{exfiltrator::SignalOnly, SignalsInfo},
};

pub fn main() -> anyhow::Result<()> {
    run_app();

    let mut signals = SignalsInfo::<SignalOnly>::new(&[SIGINT, SIGTERM, SIGQUIT])?;

    for signal in &mut signals {
        // Will print info about signal + where it comes from.
        match signal {
            SIGINT => {
                println!("got SIGINT");
                break;
            }
            SIGTERM => {
                println!("got SIGTERM");
                break;
            }
            SIGQUIT => {
                println!("got SIGQUIT but not breaking");
            }

            unknown => {
                println!("got unknown signal: {unknown}");
            }
        }
    }

    Ok(())
}

fn run_app() -> std::thread::JoinHandle<()> {
    spawn(move || {
        let mut count = 0;
        loop {
            println!("count: {count}");
            count += 1;
            sleep(Duration::from_secs(1));
        }
    })
}
