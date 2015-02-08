#![feature(io)]
#![feature(path)]
#![feature(core)]
#![feature(hash)]
#![feature(rand)]
#![feature(collections)]

use std::old_io::{Reader};
use std::old_io::fs::{self};

mod data;
mod encoding;
mod genetic;

fn main() {

    let mut test_data = data::TestData {
        data_sets: Vec::new()
    };

    println!("Reading Test Data");

    for path in fs::walk_dir(&Path::new(".")).unwrap() {
        if path.extension() == Some(b"spandat") {
            println!(" > Reading Test Data File {}", path.display());
            let data_set = data::read_span_file(&path);
            test_data.data_sets.push(data_set);
        }
    }

    genetic::compute_fitness_of_all_encodings(&test_data);

    // let mut population = vec![];

    // for _ in range(0, 30) {
    //     population.push(encoding::generate_random_encoding(32, 3));
    // }

    // for _ in range(0, 100000) {
    //     let mut ng = genetic::iterate_population(&population[], &test_data);
    //     population.clear();
    //     population.extend(ng.drain());
    // }

    // for _ in range(0, 5) {
    //     let encoding = encoding::generate_random_encoding(32, 3);
    //     println!("");
    //     println!("Testing Encoding {}", encoding.to_string());

    //     for data_set in test_data.data_sets.iter() {

    //         print!(" > Dataset '{}': ", data_set.name.filename_display());

    //         let mut encodable_count = 0;

    //         for &span in data_set.spans.iter() {
    //             if encoding.can_encode(span) {
    //                 encodable_count += 1;
    //             }
    //         }

    //         let ratio = (encodable_count as f64) / (data_set.spans.len() as f64);

    //         println!("{:.2}%", ratio * 100.0f64);
    //     }
    // }
}