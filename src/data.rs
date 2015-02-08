use std::old_io::{Reader, IoError, IoErrorKind, BufferedReader};
use std::old_io::fs::{File};
use std::mem;

#[derive(PartialEq, Eq, Copy, Debug)]
pub struct Span {
    pub position: u32,
    pub length: u32,
    pub expn_id: u32
}

pub struct SpanDataSet {
    pub spans: Vec<Span>,
    pub name: Path
}

pub fn read_span_file(path: &Path) -> SpanDataSet {

    let file = File::open(path).unwrap();
    let size = file.stat().unwrap().size;
    let mut file = BufferedReader::new(file);

    let mut spans = Vec::with_capacity((size / (mem::size_of::<Span>() as u64)) as usize);

    loop {
        let position = match file.read_le_u32() {
            Ok(position) => position,
            Err(IoError { kind: IoErrorKind::EndOfFile, .. }) => break,
            Err(IoError { desc, .. }) => panic!(desc)
        };

        let length = file.read_le_u32().unwrap();
        let expn_id = file.read_le_u32().unwrap();

        spans.push(Span {
            position: position,
            length: length,
            expn_id: expn_id
        });
    }

    SpanDataSet {
        spans: spans,
        name: path.clone()
    }
}

pub struct TestData {
    pub data_sets: Vec<SpanDataSet>
}