/*
Copyright (c) 2020 Pierre Marijon <pmarijon@hhu.de>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

/* crate use */
use anyhow::Result;

use crate::cli;

pub fn get(params: cli::SubCommandGet) -> Result<()> {
    let input = if let Some(input) = params.input {
        niffler::get_reader(Box::new(std::io::BufReader::new(
            std::fs::File::open(input).expect("Error open"),
        )))
        .expect("Error uncompress")
        .0
    } else {
        niffler::get_reader(Box::new(std::io::BufReader::new(std::io::stdin())))
            .expect("Error uncompresse")
            .0
    };

    let mut output: Box<dyn std::io::Write> = if let Some(output) = params.output {
        Box::new(std::io::BufWriter::new(
            std::fs::File::create(output).expect("Error open"),
        ))
    } else {
        Box::new(std::io::BufWriter::new(std::io::stdout()))
    };

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .flexible(true)
        .from_reader(input);

    let mut rec = csv::StringRecord::new();

    let mut begin_pos = 0;
    while reader.read_record(&mut rec).unwrap() {
        let record: PafRecord = rec.deserialize(None).expect("Error deserialize");

        let end_pos = reader.position().byte();

        writeln!(output, "{},{},{}", record.id_a, begin_pos, end_pos).expect("Error durring write");
        writeln!(output, "{},{},{}", record.id_b, begin_pos, end_pos).expect("Error durring write");

        begin_pos = end_pos;
    }

    Ok(())
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PafRecord<'a> {
    pub id_a: &'a str,
    pub length_a: usize,
    pub begin_a: u32,
    pub end_a: u32,
    pub _strand: char,
    pub id_b: &'a str,
    pub length_b: usize,
    pub begin_b: u32,
    pub end_b: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct M4Record<'a> {
    pub id_a: &'a str,
    pub id_b: &'a str,
    pub _error: f64,
    pub _shared_min: u64,
    pub _strand_a: char,
    pub begin_a: u32,
    pub end_a: u32,
    pub length_a: usize,
    pub _strand_b: char,
    pub begin_b: u32,
    pub end_b: u32,
    pub length_b: usize,
}
