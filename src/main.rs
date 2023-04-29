use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::{spawn, sleep};
use std::time::Duration;

use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
// A friend of the Signals iterator, but can be customized by what we want yielded about each
// signal.
use signal_hook::iterator::SignalsInfo;
use signal_hook::iterator::exfiltrator::SignalOnly;

fn main() -> Result<(), Error> {
    run_app().join().unwrap();

    // Make sure double CTRL+C and similar kills
    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        flag::register(*sig, term_now.clone())?;
    }

    let mut signals = SignalsInfo::<SignalOnly>::new(TERM_SIGNALS)?;

    for signal in &mut signals {
        // Will print info about signal + where it comes from.
        println!("\ngot signal: {}\n", signal);
        break;
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