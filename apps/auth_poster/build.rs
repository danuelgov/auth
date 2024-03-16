use std::{fs::File, io::Write, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    ensure_generated()?;
    create_env_file()?;

    Ok(())
}

fn ensure_generated() -> Result<(), Box<dyn std::error::Error>> {
    let generated = Path::new("src/generated");
    if !generated.exists() {
        std::fs::create_dir(generated)?;
    }

    Ok(())
}

fn create_env_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("src/generated/env.rs")?;
    writeln!(
        file,
        "pub const QUEUE_URL: &'static str = \"{}\";",
        std::env::var("QUEUE_URL")?
    )?;

    Ok(())
}
