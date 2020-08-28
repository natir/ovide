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

#[derive(clap::Clap, Debug)]
#[clap(
    version = "0.1",
    author = "Pierre Marijon <pmarijon@hhu.de>",
    about = "OvIde Ovelap Indexing tools"
)]
pub struct Command {
    #[clap(subcommand)]
    pub subcmd: SubCommand,

    #[clap(
        short = "v",
        long = "verbosity",
        parse(from_occurrences),
        about = "verbosity level also control by environment variable PCON_LOG if flag is set PCON_LOG value is ignored"
    )]
    pub verbosity: i8,
}

#[derive(clap::Clap, Debug)]
pub enum SubCommand {
    Get(SubCommandGet),
    Merge(SubCommandMerge),
}

#[derive(clap::Clap, Debug)]
#[clap(about = "Out read id and position in file")]
pub struct SubCommandGet {
    #[clap(
        short = "i",
        long = "inputs",
        about = "Path to inputs, if not set read stdin, paf format only"
    )]
    pub input: Option<String>,

    #[clap(
        short = "o",
        long = "output",
        about = "Path to outputs, if not set write in stdout"
    )]
    pub output: Option<String>,
}

#[derive(clap::Clap, Debug)]
#[clap(about = "Merge successive position of same id and produce one record per id")]
pub struct SubCommandMerge {
    #[clap(
        short = "i",
        long = "input",
        about = "Path of sorted read id to position file, if not set read stdin"
    )]
    pub input: Option<String>,

    #[clap(
        short = "o",
        long = "output",
        about = "Path to outputs, if not set read stdout"
    )]
    pub output: Option<String>,
}

pub fn i82level(level: i8) -> Option<log::Level> {
    match level {
        std::i8::MIN..=0 => None,
        1 => Some(log::Level::Error),
        2 => Some(log::Level::Warn),
        3 => Some(log::Level::Info),
        4 => Some(log::Level::Debug),
        5..=std::i8::MAX => Some(log::Level::Trace),
    }
}
