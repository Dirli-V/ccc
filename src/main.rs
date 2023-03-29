use std::{fs, vec};

use anyhow::Result;
use nom::{bytes::complete::tag, character::complete::u64, multi::separated_list0, IResult};

fn main() -> Result<()> {
    for file in fs::read_dir("./inputs")? {
        let file = file?;
        let content = fs::read_to_string(file.path())?;
        let (rest, input) = parse(&content).map_err(|e| e.to_owned())?;
        if !rest.trim().is_empty() {
            panic!("Did not read everything: {rest}");
        }
        let output = solve(input);
        let lines = out(output);
        fs::write(
            file.path()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("outputs")
                .join(file.path().file_name().unwrap()),
            lines.join("\n"),
        )?;
    }

    Ok(())
}

struct Input {
    lines: Vec<String>,
}

struct Output {
    lines: Vec<String>,
}

fn parse_line(input: &str) -> IResult<&str, i64> {
    let (input, n) = u64(input)?;

    Ok((input, (n + 1) as i64))
}

fn parse(input: &str) -> IResult<&str, Input> {
    let (input, list) = separated_list0(tag("\n"), parse_line)(input)?;
    Ok((
        input,
        Input {
            lines: list.into_iter().map(|x| x.to_string()).collect(),
        },
    ))
}

fn solve(input: Input) -> Output {
    Output { lines: input.lines }
}

fn out(output: Output) -> Vec<String> {
    let mut lines = vec![];
    for l in output.lines {
        lines.push(l);
    }
    lines
}
