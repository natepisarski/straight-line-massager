extern crate clap;
extern crate minidom;

use std::fs::File;
use std::io::Read;
use clap::{Arg, App, SubCommand};
use minidom::Element;


fn main() {
    let app = App::new("Straight Line Massager")
        .about("Turn a .gpx file into a 'massaged' CSV of latitude,longitude pairs.")
        .author("Nathaniel Pisarski")
        .arg(Arg::with_name("file"));
    let matches = app.get_matches();

    let mut file_argument = matches.value_of("file");

    if None == file_argument {
        file_argument = Some("to-convert.gpx");
    }

    let file = file_argument.unwrap();

    // The .gpx file we're going to read has the following structure:
    // <xml root>
    //   <gpx>
    //    many: <trk>
    //      many: <trkseg>
    //        many: <trkpt>
    //
    // For our purposes in straight-line missions, we just need the first Track. Diffreent attempts normally happen on different tracks.
    // We also only care about the first segment. So then we just take the first segment of the first track, and get the points out of it.
    // All points have 'lat' and 'lon' properties.

    let mut file_object = File::open(file).unwrap();
    let mut file_contents = String::new();
    file_object.read_to_string(&mut file_contents).unwrap();

    let root: Element = file_contents.parse().unwrap();

    let first_track = root.get_child("trk", "http://www.topografix.com/GPX/1/0").unwrap();
    let first_track_segment = first_track.get_child("trkseg", "http://www.topografix.com/GPX/1/0").unwrap();
    let all_track_points = first_track_segment.children();

    let mut csv_contents: Vec<(String, String)> = vec!();

    for point in all_track_points {
        let latitude = String::from(point.attr("lat").unwrap());
        let longitude = String::from(point.attr("lon").unwrap());

        csv_contents.push((latitude, longitude));
    }

    println!("Hello, world! {:#?}", csv_contents);
}
