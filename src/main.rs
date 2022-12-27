use anyhow::anyhow;

mod parser;
mod abc;
mod parse_tree;

fn main() -> Result<(), anyhow::Error> {
    let mut args = std::env::args();

    if args.len() < 2 {
        return Err(anyhow!("invalid # of arguments, needs path to abc file"));
    }
    // executable name
    args.next();

    let file_path = args.next().unwrap();

    let abc = parser::parse_abc(&file_path)?;

    println!("abc is: {:#?}", abc);

    Ok(())
}
