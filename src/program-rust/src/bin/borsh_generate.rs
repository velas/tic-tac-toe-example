
use std::{path::Path, fs};


fn generate_borsh_schema<P: AsRef<Path>>(input_path: P, schema_path: P) -> anyhow::Result<()> {
    fs::create_dir_all(&schema_path)?;
    let layouts = agsol_borsh_schema::generate_layouts(input_path)?;
    agsol_borsh_schema::generate_output(&layouts, schema_path)
}

fn main() -> anyhow::Result<()> {

    let result =generate_borsh_schema("./src/program-rust/src", "./src/client");
    println!("{:?}", result);
    result
}
