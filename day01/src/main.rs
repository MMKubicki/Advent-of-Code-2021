use clap::Parser;
use miette::{IntoDiagnostic, Result};

use std::fs;
use std::io;
use std::io::BufRead;
use std::path;

#[derive(Parser)]
struct Opts {
    #[clap(parse(from_os_str))]
    input: path::PathBuf,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Meta {
    largest: usize,
    smallest: usize,
    increases: usize,
    decreases: usize,
    count: usize,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            largest: usize::MIN,
            smallest: usize::MAX,
            increases: 0,
            decreases: 0,
            count: 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct MetaHelper<T> {
    meta: Meta,
    last: Option<T>,
}

impl<T> Default for MetaHelper<T> {
    fn default() -> Self {
        Self {
            meta: Default::default(),
            last: None,
        }
    }
}

fn first(values: &[usize]) {
    let result = values
        .iter()
        .fold(MetaHelper::default(), |mut helper, b: &usize| {
            let b: usize = *b;

            match helper.last {
                Some(last) if b > last => helper.meta.increases += 1,
                Some(last) if b < last => helper.meta.decreases += 1,
                _ => {}
            }
            match b {
                b if b > helper.meta.largest => helper.meta.largest = b,
                b if b < helper.meta.smallest => helper.meta.smallest = b,
                _ => {}
            }

            helper.last = Some(b);
            helper.meta.count += 1;

            helper
        });

    println!("Part 1:\n{:#?}", result.meta);
}

fn second(values: &[usize]) {
    let result = values
        .windows(3)
        .fold(MetaHelper::default(), |mut helper, b| {
            assert_eq!(b.len(), 3);
            let b = (b[0], b[1], b[2]);
            let b_sum = b.0 + b.1 + b.2;

            match helper.last {
                Some(a_sum) if b_sum > a_sum => helper.meta.increases += 1,
                Some(a_sum) if b_sum > a_sum => helper.meta.decreases += 1,
                _ => {}
            }
            match b_sum {
                sum if sum > helper.meta.largest => helper.meta.largest = sum,
                sum if sum < helper.meta.smallest => helper.meta.smallest = sum,
                _ => {}
            }

            helper.last = Some(b_sum);
            helper.meta.count += 1;

            helper
        });

    println!("Part 2:\n{:#?}", result.meta);
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let file = fs::File::open(opts.input).into_diagnostic()?;
    let reader = io::BufReader::new(file);
    let values = reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse())
        .filter_map(Result::ok)
        .collect::<Vec<usize>>();

    first(&values);
    second(&values);

    Ok(())
}
