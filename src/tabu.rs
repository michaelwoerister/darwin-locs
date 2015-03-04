
use std::collections::HashMap;
use encoding::{self, Encoding, SubEncoding};
use data;
use genetic;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct SolutionId(u64);


fn solution_id(sub_encodings: &[SubEncoding]) -> SolutionId {
    let mut key = 0u64;

    for sub_encoding in sub_encodings.iter() {
        key = (key << 5) | (sub_encoding.length_bits as u64);
        key = (key << 5) | (sub_encoding.position_bits as u64);
        key = (key << 5) | (sub_encoding.expn_id_bits as u64);
    }

    SolutionId(key)
}

fn get_sub_encoding_neighbours(sub_encoding: &SubEncoding,
                               output: &mut Vec<SubEncoding>)
{
    if sub_encoding.length_bits > 0 {
        output.push(SubEncoding::new(sub_encoding.length_bits - 1,
                                     sub_encoding.position_bits + 1,
                                     sub_encoding.expn_id_bits));

        output.push(SubEncoding::new(sub_encoding.length_bits - 1,
                                     sub_encoding.position_bits,
                                     sub_encoding.expn_id_bits + 1));
    }

    if sub_encoding.position_bits > 0 {
        output.push(SubEncoding::new(sub_encoding.length_bits + 1,
                                     sub_encoding.position_bits - 1,
                                     sub_encoding.expn_id_bits));

        output.push(SubEncoding::new(sub_encoding.length_bits,
                                     sub_encoding.position_bits - 1,
                                     sub_encoding.expn_id_bits + 1));
    }

    if sub_encoding.expn_id_bits > 0 {
        output.push(SubEncoding::new(sub_encoding.length_bits,
                                     sub_encoding.position_bits + 1,
                                     sub_encoding.expn_id_bits - 1));

        output.push(SubEncoding::new(sub_encoding.length_bits + 1,
                                     sub_encoding.position_bits,
                                     sub_encoding.expn_id_bits - 1));
    }
}

fn get_encoding_neighbours(encoding: &Encoding,
                           output: &mut Vec<(Encoding, SolutionId)>,
                           tabu_list: &HashMap<SolutionId, i64>) {

    let mut sub_encodings: Vec<SubEncoding> = vec![];
    let mut neighbours: Vec<SubEncoding> = vec![];

    for (i, sub_encoding) in encoding.sub_encodings.iter().enumerate() {
        // Reset encoding to initial state
        sub_encodings.clear();
        sub_encodings.extend(encoding.sub_encodings.iter().map(|x| *x));

        // generate neighbour list
        neighbours.clear();
        get_sub_encoding_neighbours(sub_encoding, &mut neighbours);

        for neighbour in neighbours.iter() {
            sub_encodings[i] = *neighbour;
            let solution_id = solution_id(&sub_encodings[..]);

            if !tabu_list.contains_key(&solution_id) {
                output.push((Encoding::new(&sub_encodings[..], encoding.max_total_bit_count()),
                             solution_id));
            }
        }
    }
}

pub fn run_tabu_search(
    bit_count: u64,
    sub_encoding_bits: u64,
    test_data: &data::TestData,
    tag: String) -> (Encoding, f64, String)
{

    let mut tabu_list: HashMap<SolutionId, i64> = HashMap::new();

    let sub_encodings_per_encoding = (1 << (bit_count - sub_encoding_bits)) - 1;

    let mut best_solution = encoding::generate_random_encoding(bit_count, sub_encodings_per_encoding);
    let mut best_fitness = genetic::compute_fitness(&best_solution, test_data);

    let mut neighbours: Vec<(Encoding, SolutionId)> = vec![];
    let mut neighbour_fitness: Vec<f64> = vec![];

    let mut current = best_solution.clone();

    for iteration in range(1i64, 1000000) {
        neighbours.clear();
        neighbour_fitness.clear();

        get_encoding_neighbours(&current, &mut neighbours, &tabu_list);

        if neighbours.len() == 0 {
            return (best_solution, best_fitness, tag);
        }

        for &(ref neighbour, solution_id) in neighbours.iter() {
            tabu_list.insert(solution_id, iteration);
            neighbour_fitness.push(genetic::compute_fitness(neighbour, test_data));
        }

        let mut best_index = 0;
        let mut best_neighbour_fitness = neighbour_fitness[0];

        for i in range(1, neighbour_fitness.len()) {
            if neighbour_fitness[i] > best_neighbour_fitness {
                best_index = i;
                best_neighbour_fitness = neighbour_fitness[i];
            }
        }

        if best_neighbour_fitness > best_fitness {
            best_solution = neighbours[best_index].0.clone();
            best_fitness = best_neighbour_fitness;
        }

        current = neighbours[best_index].0.clone();

        if (iteration & 511) == 0 {
            println!("{}. cleaning tabu_list. best so far: {} (fitness={})", tag,
                best_solution.to_string(), best_fitness);

            let iteration_limit = iteration - 300;

            let mut keys = vec![];

            for (key, value) in tabu_list.iter() {
                if *value < iteration_limit {
                    keys.push(*key);
                }
            }

            for key in keys {
                tabu_list.remove(&key);
            }
        }
    }

    return (best_solution, best_fitness, tag);
}
