use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let queue_url = std::env::var("QUEUE_URL")?;

    let mut file = std::fs::File::create("src/generated/env.rs")?;
    writeln!(
        file,
        "pub const QUEUE_URL: &'static str = \"{}\";",
        queue_url
    )?;

    Ok(())
}
