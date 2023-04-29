use serde::Deserialize;

mod sync;
mod tokio;

#[derive(Deserialize)]
struct Env {
    #[serde(default)]
    mode: Mode,
}

fn main() -> anyhow::Result<()> {
    let Env { mode } = envy::from_env()?;
    println!("\nmode: {:?}\n", mode);
    match mode {
        Mode::Sync => sync::main(),
        Mode::Tokio => tokio::main(),
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug, Default)]
enum Mode {
    #[default]
    Sync,
    Tokio,
}
