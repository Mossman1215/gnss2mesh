use clap::Parser;
use patharg::{InputArg, OutputArg};
use wavefront_rs::obj;
use std::{error::Error, io::Write};

#[derive(Parser)]
struct Arguments {
    /// The file to write the case-flipped text to.
    #[arg(short = 'o', long, default_value_t)]
    // The `default_value_t` attribute causes the default value of the argument
    // to be `OutputArg::default()`, which equals `OutputArg::Stdout`.
    outfile: OutputArg,

    /// The file to read the text to case-flip from.
    #[arg(default_value_t)]
    // The `default_value_t` attribute causes the default value of the argument
    // to be `InputArg::default()`, which equals `InputArg::Stdin`.
    infile: InputArg,
}


fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    let mut output = args.outfile.create()?;
    writeln!(&mut output, "{}", convert_mesh(args.infile))?;
    Ok(())
}

/**
 * use EPSG:2135 for UTM data
 * use EPSG:4167 nzgd2000 to convert geojson measurements to xyz
 */
fn convert_mesh (inputfile: InputArg) -> Result<obj,&str>{
    Err("not implemented")
}

fn convert_pt (pt: geo::Point) -> Result< Entity::Vertex,&str>{
    Err("not implemented")
}