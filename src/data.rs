use std::old_io::{Reader, IoError, IoErrorKind, BufferedReader};
use std::old_io::fs::{File};
use std::mem;

#[derive(PartialEq, Eq, Copy, Debug, Hash)]
pub struct Span {
    pub position: u64,
    pub length: u64,
    pub expn_id: u64
}

pub struct SpanDataSet {
    pub expansions: Vec<(i64, i64)>,
    pub spans: Vec<Span>,
    pub name: String
}

pub fn read_span_file(path: &Path) -> SpanDataSet {

    let file = File::open(path).unwrap();
    let mut size = file.stat().unwrap().size;

    let mut file = BufferedReader::new(file);

    let expansion_count = file.read_le_i32().unwrap() as usize;
    let mut expansions = Vec::with_capacity(expansion_count);

    for _ in 0..expansion_count {
        let lo = match file.read_le_i64() {
            Ok(size) => size,
            Err(IoError { desc, .. }) => panic!(desc)
        };

        let hi = match file.read_le_i64() {
            Ok(size) => size,
            Err(IoError { desc, .. }) => panic!(desc)
        };

        expansions.push((lo as i64, hi as i64));
    }

    size -= (mem::size_of::<i32>() * expansion_count) as u64;
    let mut spans = Vec::with_capacity((size / (mem::size_of::<Span>() as u64)) as usize);
    let mut null_spans: u64 = 0;

    loop {
        let position = match file.read_le_u32() {
            Ok(position) => position,
            Err(IoError { kind: IoErrorKind::EndOfFile, .. }) => break,
            Err(IoError { desc, .. }) => panic!(desc)
        };

        let length = file.read_le_u32().unwrap();
        let expn_id = file.read_le_u32().unwrap();

        let span = Span {
            position: position as u64,
            length: length as u64,
            expn_id: expn_id as u64
        };

        if length == 0 {
            null_spans += 1;
        }

        spans.push(span);
    }

    let null_spans_percent = ((100 * null_spans) as f64 / spans.len() as f64) as u64;
    println!("Null spans: {}%", null_spans_percent);

    SpanDataSet {
        expansions: expansions,
        spans: spans,
        name: format!("{}", path.display())
    }
}

pub struct TestData {
    pub data_sets: Vec<SpanDataSet>
}