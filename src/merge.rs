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

pub fn merge(params: cli::SubCommandMerge) -> Result<()> {
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
        .delimiter(b',')
        .has_headers(false)
        .flexible(true)
        .from_reader(input);

    let mut rec = csv::StringRecord::new();

    let mut actual_id = String::new();
    let mut start = 0;
    let mut stop = 0;
    let mut offsets: Vec<(u64, u64)> = Vec::new();
    while reader.read_record(&mut rec).unwrap() {
        let record: GetRecord = rec.deserialize(None).expect("Error deserialize");

        if actual_id.is_empty() {
            actual_id = record.id.to_string();
            start = record.begin;
            stop = record.end;
        }

        if actual_id != record.id {
            write!(output, "{}", actual_id).expect("Error durring write");
            offsets.push((start, stop));
            for offset in offsets.iter() {
                write!(output, ",{},{}", offset.0, offset.1).expect("Error durring write");
            }
            writeln!(output, "").expect("Error durring write");

            actual_id = record.id.to_string();
            start = record.begin;
            stop = record.end;
            offsets.clear();
        }

        if stop != record.end && stop != record.begin {
            offsets.push((start, stop));
            start = record.begin;
            stop = record.end;
        } else {
            stop = record.end;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GetRecord<'a> {
    pub id: &'a str,
    pub begin: u64,
    pub end: u64,
}
