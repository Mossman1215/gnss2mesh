use clap::Parser;
use geo::{line_measures, Distance};
use geo::{Coord, CoordinatePosition, Point};
use patharg::{InputArg, OutputArg};
use shapefile::record::EsriShape;
use shapefile::{Shape, NO_DATA};
use shapefile::PointZ;
use wavefront_rs::obj::entity::Entity::Vertex;
use wavefront_rs::obj::{self, entity::Entity, format_writer::FormatWriter};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fs::File;
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
    let mesh = convert_mesh(args.infile);
    for vertex in  mesh{
        FormatWriter::write(output.by_ref(), &vertex);
        writeln!(output.by_ref())?;
    }

    Ok(())
}

/**
 * use EPSG:2135 for UTM data
 * use EPSG:4167 nzgd2000 to convert geojson measurements to xyz
 * fetch feature 0
 * let feat_zero = feature_set.features[0].geometry.clone().unwrap(); 
 * foreach feature
 *  for feature in feature_set.features {
 *      let point: Point<f64> = geo_types::Point::try_from(feature.geometry.unwrap().clone()).unwrap();
 *  }
 *   apply geodesic distance between this point and feature 0
 *   save to a new vertex in mutable wavefront entity
 * return entity or a copy of entity
 */
fn convert_mesh (inputfile: InputArg) -> Vec<Entity>{
    // create a mutable wavefront entity collection
    let mut wavefront_ent = Vec::new();
    let mut reader = shapefile::Reader::from_path(inputfile.into_path().expect("input path failed")).expect("failed to create reader");
    let mut readervec = reader.iter_shapes_and_records();
    let (shape_zero, record_zero) = readervec.nth(0).unwrap().expect("failed to make shape");
    let point_zero = shapefile::PointZ::try_from(shape_zero).expect("failed to convert to pointZ");
    for shape_record in  readervec{
        let (shape, record) = shape_record.expect("failed to make shape record");
        //println!("shape: {}", shape);
        let pointy = shapefile::PointZ::try_from(shape).expect("cannot convert to pointZ");
        let delta_point = distance_calculation(point_zero, pointy);
        let vert = Vertex{x: delta_point.x, y: delta_point.y, z:delta_point.z,w: None};
        //println!("vert: {}",vert.to_string());
        wavefront_ent.push(vert);
    }

    wavefront_ent
}
/**
 * calculate component distances from pt1 to pt2 from lon,lat (hopefully into meters?)
 */
fn distance_calculation(pt1: PointZ, pt2: PointZ) -> PointZ{
    //x = measure((pt1.x, pt1.y, pt1.z) (pt2.x, pt1.y, pt1.z))
    let x = line_measures::Geodesic::distance(Point::new(pt1.x, pt1.y), Point::new(pt2.x, pt1.y));
    let y = line_measures::Geodesic::distance(Point::new(pt1.x, pt1.y), Point::new(pt1.x, pt2.y));
    let z = pt2.z - pt1.z;
    return PointZ::new(x, y,z,NO_DATA);
}