use itertools::Itertools;
use rayon::prelude::*;
use std::fmt;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process::{Command, Stdio};
use urbit_ob::*;

fn validate_patp(point_num: &str, expected_patp: &str) -> Result<bool, Error> {
    Ok(dec2patp(point_num)? == expected_patp && patp2dec(expected_patp)? == point_num)
}

fn validate_patq(point_num: &str, expected_patq: &str) -> Result<bool, Error> {
    Ok(dec2patq(point_num)? == expected_patq && patq2dec(expected_patq)? == point_num)
}

#[derive(Debug, Copy, Clone)]
struct WorkReport {
    start: u32,
    end: u32,
    success_count: usize,
}

impl fmt::Display for WorkReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Start: 0x{:08x}, End: 0x{:08x}, Success Count: 0x{:08x}",
            self.start, self.end, self.success_count
        )
    }
}

fn main() -> io::Result<()> {
    (0..0xfff)
        .into_par_iter()
        .map(|n| {
            let start = n << 20 | 0;
            let end = n << 20 | 0xfffff;
            let mut node_cmd = Command::new("node")
                .arg("index.js")
                .arg("patp")
                .arg(format!("{}", start))
                .arg(format!("{}", end))
                .stdout(Stdio::piped())
                .spawn()?;

            let stdout = node_cmd.stdout.take().ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Couldn't get stdout from node",
            ))?;

            let mut good_rows = 0;
            for line_result in BufReader::new(stdout).lines() {
                let line = line_result?;
                let (point_num, expected_patp) = line
                    .trim()
                    .split_whitespace()
                    .collect_tuple()
                    .ok_or(io::Error::new(
                        io::ErrorKind::Other,
                        "Badly formatted input line",
                    ))?;
                let valid = validate_patp(point_num, expected_patp)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "patp calculation error"))?;
                if valid {
                    good_rows += 1;
                } else {
                    println!("MISMATCH: {}", line);
                }
            }

            node_cmd.wait()?;

            Ok(WorkReport {
                start,
                end,
                success_count: good_rows,
            })
        })
        .for_each(|r: Result<WorkReport, io::Error>| match r {
            Ok(report) => {
                println!("{}", report);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        });
    Ok(())
}
