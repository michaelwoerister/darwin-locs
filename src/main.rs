#![feature(core)]
#![feature(rand)]
#![feature(old_path)]
#![feature(old_io)]

use std::old_io::{Reader};
use std::old_io::fs::{self};
use std::thread;
use std::mem;

mod data;
mod encoding;
mod genetic;
mod tabu;
mod no_exp_id_encoding;

fn main() {

    let mut test_data = data::TestData {
        data_sets: Vec::new()
    };

    let mut test_data_transformed = data::TestData {
        data_sets: Vec::new()
    };

    println!("Reading Test Data");

    for path in fs::walk_dir(&Path::new(".")).unwrap() {
        if path.extension() == Some(b"spandat") {
            println!(" > Reading Test Data File {}", path.display());
            let data_set = data::read_span_file(&path);

            let transformed_dataset =
                no_exp_id_encoding::transform_data_to_non_expansion_id_form(&data_set);

            test_data.data_sets.push(data_set);
            test_data_transformed.data_sets.push(transformed_dataset);
        }
    }

    let thread_count = 4;
    let mut results = vec![];

    for iteration in range(0, 100) {
        let mut guards = vec![];

        for thread_index in range(0, thread_count) {

            if thread_index & 1 == 0 {
                let test_data_ref: &'static data::TestData = unsafe {
                    mem::transmute(&test_data)
                };

                guards.push(thread::scoped(move || {
                    let tag = format!("(w/ expn_id) Thread {}", thread_index);
                    tabu::run_tabu_search(32, 31, test_data_ref, tag)
                }));
            } else {
                let test_data_ref: &'static data::TestData = unsafe {
                    mem::transmute(&test_data_transformed)
                };

                guards.push(thread::scoped(move || {
                    let tag = format!("(w/o expn_id) Thread {}", thread_index);
                    tabu::run_tabu_search(32, 30, test_data_ref, tag)
                }));
            }
        }

        for guard in guards {
            results.push(guard.join());
        }

        println!("iteration {}, best results so far:", iteration);
        results.dedup();
        results.as_mut_slice()
               .sort_by(|&(_, f1, _), &(_, f2, _)| {
                    if f1 > f2 { ::std::cmp::Ordering::Less } else { ::std::cmp::Ordering::Greater }
                });
        results.truncate(5);

        for result in results.iter() {
            println!("{}: {}, fitness={}", result.2, encoding::Encoding::to_string(&result.0), result.1);
        }
    }

    println!("best results:");
    for result in results {
        println!("{}, fitness={}", encoding::Encoding::to_string(&result.0), result.1);
    }
}