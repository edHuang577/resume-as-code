use std::error::Error;
use std::path::Path;
use std::result::Result;

use protoc_rust;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to proto definition directory containing 'resume.proto'
    #[structopt(short, long)]
    proto_dir: String,
    /// Directory of rust crate to output compiled proto models
    #[structopt(short, long)]
    crate_dir: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let crate_dir = Path::new(&opt.crate_dir).join("src/protos");
    let proto_path = Path::new(&opt.proto_dir).join("resume.proto");

    println!("Generating proto code at '{:?}'", &crate_dir);
    protoc_rust::Codegen::new()
        .out_dir(crate_dir.to_str().unwrap())
        .inputs(&[proto_path.to_str().unwrap()])
        .include(&opt.proto_dir)
        .customize(protoc_rust::Customize {
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()?;

    Ok(())
}
