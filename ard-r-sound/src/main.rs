use clap::Parser;
use tracing::info;

mod codegen;
mod parse_tree;
mod parser;
mod player;
mod abc;
mod args;

fn main() -> Result<(), anyhow::Error> {
    let args = args::Args::parse();

    tracing_subscriber::fmt::init();

    let file_path = args.input_file();

    let abc = parser::parse_abc_file(&file_path)?;

    info!("abc is: {:#?}", abc);

    match args.file_format() {
        Some(args::FileFormat::Raw) => player::write_as_raw(abc, args.output_file()?)?,
        Some(args::FileFormat::Wav) => player::write_as_wav(abc, args.output_file()?)?,
        Some(args::FileFormat::Play) => player::play(abc)?,
        Some(args::FileFormat::Header) => codegen::generate_c_header(&abc, args.output_file()?)?,
        None => {}
    }

    Ok(())
}
