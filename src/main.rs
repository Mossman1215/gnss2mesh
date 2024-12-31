use clap::Parser;
use geo::{Coord, CoordinatePosition, Point};
use patharg::{InputArg, OutputArg};
use wavefront_rs::obj::{self, entity::Entity};
use std::{error::Error, io::Write, str::FromStr};

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
    writeln!(&mut output, "{:?}", convert_mesh(args.infile))?;
    Ok(())
}

/**
 * use EPSG:2135 for UTM data
 * use EPSG:4167 nzgd2000 to convert geojson measurements to xyz
 */
fn convert_mesh (inputfile: InputArg) -> Result<Entity,String>{
    let mut reader = shapefile::Reader::from_path(inputfile.into_path().expect("input path failed")).expect("failed to create reader");
    for shape_record in reader.iter_shapes_and_records() {
        let (shape, record) = shape_record.expect("failed to make record");
        println!("{}", shape);
    }
    
    // create a mutable wavefront entity
    let mut wavefront_ent = wavefront_rs::obj::entity::Entity::Object { name: ("landscape".to_string()) };
    //fetch feature 0
    //let feat_zero = feature_set.features[0].geometry.clone().unwrap(); 
    //foreach feature
    // for feature in feature_set.features {
    //     let point: Point<f64> = geo_types::Point::try_from(feature.geometry.unwrap().clone()).unwrap();
    // }
    //  apply geodesic distance between this point and feature 0
    //  save to a new vertex in mutable wavefront entity
    
    //return entity or a copy of entity
    Err(String::from("not implemented"))
}