use clap::Parser;

mod abc;
mod args;
mod parse_tree;
mod parser;
mod player;

fn main() -> Result<(), anyhow::Error> {
    let args = args::Args::parse();

    let file_path = args.input_file();

    let abc = parser::parse_abc(&file_path)?;

    println!("abc is: {:#?}", abc);

    match args.file_format() {
        Some(args::FileFormat::Raw) => player::write_as_raw(abc, args.output_file()?)?,
        Some(args::FileFormat::Wav) => player::write_as_wav(abc, args.output_file()?)?,
        Some(args::FileFormat::Play) => player::play(abc)?,
        None => {}
    }

    Ok(())
}
