
use data::{SpanDataSet, Span};


pub fn transform_data_to_non_expansion_id_form(data: &SpanDataSet) -> SpanDataSet {

    let initial_codemap_size = data.spans
                                   .iter()
                                   .map(|sp| sp.position + sp.length)
                                   .max()
                                   .unwrap();

    let mut expansion_filemaps = Vec::with_capacity(data.expansions.len());

    let mut begin = initial_codemap_size;

    // allocate a new "filemap" for each expansion
    for expansion in data.expansions.iter() {
        let expansion_filemap_range = if expansion.0 < 0 {
            (begin, begin + 1)
        } else {
            assert!(((expansion.1 as i64) - (expansion.0 as i64)) > 0);

            (begin, begin + (expansion.1 - expansion.0) as u64)
        };

        expansion_filemaps.push(expansion_filemap_range);
        begin = expansion_filemap_range.1;
    }

    println!("Address space: regular 0..{}, expansions {}..{}",
        initial_codemap_size,
        initial_codemap_size,
        begin
    );

    let mut transformed_spans = Vec::with_capacity(data.spans.len());

    for span in data.spans.iter() {
        if span.position == 0 && span.length == 0 {
            transformed_spans.push(Span {
                position: 0,
                length: 0,
                expn_id: 0xFFFFFFFF
            });
        } else if span.expn_id != 0xFFFFFFFF {
            let local_lo = span.position - data.expansions[span.expn_id as usize].0 as u64;

            let transformed_span = Span {
                position: expansion_filemaps[span.expn_id as usize].0 + local_lo,
                length: span.length,
                expn_id: 0xFFFFFFFF
            };

            transformed_spans.push(transformed_span);
        } else {
            transformed_spans.push(*span);
        }
    }

    SpanDataSet {
        expansions: vec![],
        spans: transformed_spans,
        name: data.name.clone() + " (no_expn_ids)"
    }
}


// pub fn analyze_as_transformed(data: &SpanDataSet) {
//     let transformed = transform_data_to_non_expansion_id_form(data);

//     println!("Data Set: {}", data.name);
//     println!("Expansion Count = {}", data.expansions.len());

//     let codemap_size = transformed.spans.iter()
//                                   .map(|sp| sp.position + sp.length)
//                                   .max()
//                                   .unwrap();


//     for i in 0..65 {
//         if codemap_size < (1 << i) {
//             println!("Bits needed per component: {}", i + 1);
//             break;
//         }
//     }
// }
