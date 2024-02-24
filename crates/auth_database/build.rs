#[path = "build/mod.rs"]
mod build;

use build::*;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generated = std::fs::File::create("src/generated.rs")?;
    writeln!(generated, r#"// !"#)?;
    writeln!(generated, r#"// ! This file is generated."#)?;
    writeln!(generated, r#"// ! Do not modify it manually."#)?;
    writeln!(generated, r#"// ! Regenerate it by running `cargo build`."#)?;
    writeln!(generated, r#"// !"#)?;

    let mut paths: Vec<_> = std::fs::read_dir("schema")?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect();
    paths.sort();
    for path in paths {
        let source = std::fs::read_to_string(path)?;
        let schema: Schema = toml::from_str(&source)?;
        SchemaVisitor::new(&mut generated).visit_schema(&schema)?;
    }

    Ok(())
}
