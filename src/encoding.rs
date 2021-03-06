
use data::Span;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use std::slice;
use std::rand;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Clone)]
pub struct Encoding {
    pub sub_encodings: Vec<SubEncoding>,
    max_bits_for_subencodings: u64,
    max_total_bit_count: u64,
}

impl Encoding {
    pub fn new(
        sub_encodings: &[SubEncoding],
        max_total_bit_count: u64)
    -> Encoding {

        let sub_encoding_disr_bits = sub_encoding_disr_bits(sub_encodings.len() as u64);

        if sub_encodings.iter()
                        .map(|se| se.total_bit_count() + sub_encoding_disr_bits)
                        .any(|total_bit_count| total_bit_count > max_total_bit_count) {
            panic!("Invalid encoding -- max total bits exceeded.")
        }

        let mut sub_encodings = sub_encodings.to_vec();

        sub_encodings.as_mut_slice().sort();

        Encoding {
            sub_encodings: sub_encodings,
            max_bits_for_subencodings: max_total_bit_count - sub_encoding_disr_bits,
            max_total_bit_count: max_total_bit_count,
        }
    }

    pub fn can_encode(&self, span: Span) -> bool {
        for sub_encoding in self.sub_encodings.iter() {
            if sub_encoding.can_encode(span) {
                return true;
            }
        }
        return false;
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for sub_encoding in self.sub_encodings.iter() {
            s.push_str(format!("(len: {}, pos: {}, exp: {}) ",
                               sub_encoding.length_bits,
                               sub_encoding.position_bits,
                               sub_encoding.expn_id_bits).as_slice());
        }

        s
    }

    // pub fn sub_encodings(&self) -> &[SubEncoding] {
    //     &self.sub_encodings[..]
    // }

    // pub fn max_bits_for_subencodings(&self) -> u64 {
    //     self.max_bits_for_subencodings
    // }

    pub fn max_total_bit_count(&self) -> u64 {
        self.max_total_bit_count
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubEncoding {
    pub length_bits: u64,
    pub position_bits: u64,
    pub expn_id_bits: u64,
    pub max_encodable_length: u64,
    pub max_encodable_position: u64,
    pub max_encodable_expn_id: u64,
}

impl SubEncoding {
    pub fn new(
        length_bits: u64,
        position_bits: u64,
        expn_id_bits: u64)
    -> SubEncoding {
        SubEncoding {
            length_bits: length_bits,
            position_bits: position_bits,
            expn_id_bits: expn_id_bits,
            max_encodable_length: (1 << length_bits) - 1,
            max_encodable_position: (1 << position_bits) - 1,
            max_encodable_expn_id: (1 << expn_id_bits) - 1,
        }
    }

    pub fn can_encode(&self, span: Span) -> bool {
        span.position <= self.max_encodable_position &&
        span.length <= self.max_encodable_length &&
        (span.expn_id == 0xFFFFFFFF || span.expn_id <= self.max_encodable_expn_id)
    }

    pub fn total_bit_count(&self) -> u64 {
        self.length_bits + self.position_bits + self.expn_id_bits
    }
}

fn generate_all_sub_encodings(bit_count: u64) -> Vec<SubEncoding> {

    let mut sub_encodings = vec![];

    for expn_id_bits in range(0, bit_count + 1) {
        let bits_left = bit_count - expn_id_bits;

        for position_bits in range(0, bits_left  + 1) {
            let length_bits = bits_left - position_bits;
            sub_encodings.push(SubEncoding::new(length_bits, position_bits, expn_id_bits));
        }
    }

    println!("Generated extensive list of sub-encodings with {} bits: {} candidates",
             bit_count,
             sub_encodings.len());

    return sub_encodings;
}

pub fn get_all_sub_encodings(bit_count: u64) -> &'static [SubEncoding] {

    static mut ALL_SUB_ENCODINGS: [(usize, *const SubEncoding); 64] = [(0, 0 as *const SubEncoding); 64];
    static START: Once = ONCE_INIT;

    START.call_once(|| {
        for i in range(1, 64) {
            let all_sub_encodings = generate_all_sub_encodings(i as u64);
            unsafe {
                ALL_SUB_ENCODINGS[i] = (all_sub_encodings.len(), all_sub_encodings.as_slice().as_ptr());
                mem::forget(all_sub_encodings);
            }
        }
    });

    unsafe {
        return slice::from_raw_parts(ALL_SUB_ENCODINGS[bit_count as usize].1,
                                     ALL_SUB_ENCODINGS[bit_count as usize].0);
    }
}

pub fn generate_random_encoding(
    total_bit_count: u64,
    sub_encoding_count: u64)
-> Encoding {
    let sub_encoding_disr_bits = sub_encoding_disr_bits(sub_encoding_count);
    let sub_encoding_bit_count = total_bit_count - sub_encoding_disr_bits;

    let all_sub_encodings = get_all_sub_encodings(sub_encoding_bit_count);

    let mut sub_encodings = HashSet::new();

    while sub_encodings.len() < (sub_encoding_count as usize) {
        let index = (all_sub_encodings.len() as f64 * rand::random::<f64>()) as usize;
        sub_encodings.insert(all_sub_encodings[index]);
    }

    let sub_encodings: Vec<SubEncoding> = FromIterator::from_iter(sub_encodings.into_iter());

    Encoding::new(&sub_encodings[..], total_bit_count)
}

fn sub_encoding_disr_bits(sub_encoding_count: u64) -> u64 {
    match sub_encoding_count {
        0 => panic!("Invalid encoding -- no sub-encodings."),
        1 => 1,
        2 ... 3 => 2,
        4 ... 7 => 3,
        8 ... 15 => 4,
        16 ... 31 => 5,
        32 ... 63 => 6,
        _ => panic!("Invalid encoding -- too many sub-encodings.")
    }
}