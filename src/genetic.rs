
use encoding::{Encoding};
// use std::rand;
// use std::iter::FromIterator;
use data::TestData;
// use std::cmp::Ordering::{Less, Greater};

pub fn compute_fitness(encoding: &Encoding, test_data: &TestData) -> f64 {
    let mut encodable_count = 0;
    let mut total_count = 0.0;

    // print!("compute fitness for {}: ", encoding.to_string());

    for data_set in test_data.data_sets.iter() {

        for &span in data_set.spans.iter() {
            if encoding.can_encode(span) {
                encodable_count += 1;
            }
        }

        total_count += data_set.spans.len() as f64;
    }

    let fitness = (encodable_count as f64) / total_count;

    return fitness;
}

// fn select(
//     individuals: &[Encoding],
//     fitness: &[f64],
//     ratio_best: f64,
//     ratio_random: f64)
// -> Vec<Encoding> {
//     assert!(ratio_best + ratio_random < 1.0);
//     assert!(individuals.len() == fitness.len());

//     let mut zipped: Vec<(Encoding, f64)> = individuals.iter().map(|e| (*e).clone()).zip(fitness.iter().map(|x| *x)).collect();
//     zipped.as_mut_slice().sort_by(|&(_, f1), &(_, f2)| if f1 < f2 { Greater } else { Less });

//     let best_limit = (zipped.len() as f64 * ratio_best) as usize;

//     let mut result: Vec<Encoding> = FromIterator::from_iter(
//         zipped.iter()
//               .map(|&(ref e, _)| e.clone())
//               .take(best_limit));

//     let random_count = (zipped.len() as f64 * ratio_random) as usize;

//     for _ in range(0, random_count) {
//         let r = rand::random::<f64>();

//         let index = best_limit + ((zipped.len() - best_limit) as f64 * r) as usize;
//         result.push(zipped[index].0.clone());
//     }

//     println!("best in selection (fitness = {}): {} ", zipped[0].1, zipped[0].0.to_string());

//     return result;
// }

// fn next_generation(parents: &[Encoding], count: usize) -> Vec<Encoding> {

//     let mut ng = Vec::with_capacity(count);

//     ng.extend(parents.iter().take(3).map(|x| (*x).clone()));

//     for _ in range(0, count - 3) {
//         let p1 = rand_between(0, parents.len());
//         let p2 = rand_between(0, parents.len());

//         let max_bits_for_subencodings = parents[p1].max_bits_for_subencodings();
//         assert!(max_bits_for_subencodings == parents[p2].max_bits_for_subencodings());
//         assert!(parents[p1].max_total_bit_count() == parents[p2].max_total_bit_count());

//         let mut sub_encodings = vec![];

//         for (s1, s2) in parents[p1].sub_encodings().iter().zip(parents[p2].sub_encodings().iter()) {
//             let mut length_bits = (s1.length_bits + s2.length_bits) / 2;
//             let mut position_bits = (s1.position_bits + s2.position_bits) / 2;
//             let mut expn_id_bits = (s1.expn_id_bits + s2.expn_id_bits) / 2;

//             while length_bits + position_bits + expn_id_bits < max_bits_for_subencodings {
//                 match rand_between(0, 3) {
//                     0 => length_bits += 1,
//                     1 => position_bits += 1,
//                     _ => expn_id_bits += 1,
//                 }
//             }

//             sub_encodings.push(SubEncoding::new(length_bits, position_bits, expn_id_bits));
//         }

//         ng.push(Encoding::new(&sub_encodings[..], parents[p1].max_total_bit_count()));
//     }

//     for encoding in ng.iter_mut().skip(3) {
//         if rand_between(0, 100) < 15 {
//             let max_bits_for_subencodings = encoding.max_bits_for_subencodings();

//             for sub_encoding in encoding.sub_encodings.iter_mut() {

//                 sub_encoding.length_bits -= rand_between(0, sub_encoding.length_bits as usize / 2) as u64;
//                 sub_encoding.position_bits -= rand_between(0, sub_encoding.position_bits as usize / 2) as u64;
//                 sub_encoding.expn_id_bits -= rand_between(0, sub_encoding.expn_id_bits as usize / 2) as u64;

//                 while sub_encoding.length_bits + sub_encoding.position_bits + sub_encoding.expn_id_bits < max_bits_for_subencodings {
//                     match rand_between(0, 3) {
//                         0 => sub_encoding.length_bits += 1,
//                         1 => sub_encoding.position_bits += 1,
//                         _ => sub_encoding.expn_id_bits += 1,
//                     }
//                 }
//             }
//         }
//     }

//     return ng;
// }

// pub fn iterate_population(population: &[Encoding], test_data: &TestData) -> Vec<Encoding> {

//     // println!("iterating");
//     let mut fitness = vec![];

//     for encoding in population.iter() {
//         fitness.push(compute_fitness(encoding, test_data));
//     }

//     // println!("select ...");
//     let selection = select(population, &fitness[..], 0.15, 0.3);

//     // println!("next generation ...");
//     return next_generation(&selection[..], population.len());
// }

// fn rand_between(lower: usize, upper: usize) -> usize {
//     let r = rand::random::<f64>();
//     return lower + ((upper - lower) as f64 * r) as usize;
// }

// pub fn compute_fitness_of_all_encodings(test_data: &TestData,
//                                         test_range: (usize, usize),
//                                         message: &str) -> (Encoding, f64) {

//     let all_sub_encodings = encoding::get_all_sub_encodings(31);

//     // let total_count = 20214480u64;
//     let test_range_len = test_range.1 - test_range.0;

//     println!("Testing a total of {} encodings", test_range_len);

//     let mut index = 0usize;
//     let mut best_fitness = 0.0f64;
//     let mut best_encoding = None;

//     for i1 in range(0, all_sub_encodings.len()) {
//         // for i2 in range(i1+1, all_sub_encodings.len()) {
//         //     for i3 in range(i2+1, all_sub_encodings.len()) {
//                 if index < test_range.0 {
//                     index += 1;
//                     continue;
//                 }

//                 let sub_encodings = [all_sub_encodings[i1]];
//                                      // all_sub_encodings[i2],
//                                      // all_sub_encodings[i3]];

//                 let encoding = Encoding::new(&sub_encodings, 32);

//                 let fitness = compute_fitness(&encoding, test_data);

//                 if fitness > best_fitness {
//                     best_fitness = fitness;
//                     best_encoding = Some(encoding);
//                 }

//                 index += 1;

//                 if index >= test_range.1 {
//                     println!("exiting {}", message);
//                     return (best_encoding.unwrap(), best_fitness);
//                 }

//                 if (index & ((1 << 12) - 1)) == 0 {
//                     let tested_count = index - test_range.0;
//                     let percent = (((tested_count as f64) / (test_range_len as f64)) * 100.0) as u64;

//                     println!("{}. Tested {} encodings so far ({}%). Best is {} (fitness={}).",
//                              message,
//                              tested_count,
//                              percent,
//                              best_encoding.as_ref().unwrap().to_string(),
//                              best_fitness);
//                 }
//         //     }
//         // }
//     }

//     (best_encoding.unwrap(), best_fitness)
// }