use anyhow::anyhow;

mod abc;
mod parse_tree;
mod parser;
mod player;

fn main() -> Result<(), anyhow::Error> {
    let mut args = std::env::args();

    if args.len() < 2 {
        return Err(anyhow!("invalid # of arguments, needs path to abc file"));
    }
    // skip executable name
    args.next();

    let file_path = args.next().unwrap();

    let abc = parser::parse_abc(&file_path)?;

    println!("abc is: {:#?}", abc);

    player::write_output_to_file(abc.clone(), "test.pcm")?;
    player::play(abc.clone())?;

    Ok(())
}
