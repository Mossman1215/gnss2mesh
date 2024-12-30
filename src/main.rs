use clap::Parser;
use geojson::{FeatureCollection, GeoJson, Geometry, Value};
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
    let geojson_str = inputfile.read_to_string().expect("failed to parse geojson arg");
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    let feature_set = FeatureCollection::try_from(geojson).unwrap();
    println!("feature 0: {}",feature_set.features[0].geometry.as_ref().unwrap().to_string());
    // read property data
    // assert_eq!("donuts", feature_set.property("food").unwrap());

    // read geometry data
    // let geometry: Geometry = feature_set.geometry.unwrap();
    // if let Value::Point(coords) = geometry.value {
    //     assert_eq!(coords, vec![-118.2836, 34.0956]);
    // }

    Err(String::from("not implemented"))
}