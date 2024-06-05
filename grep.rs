use std::fs;

use anyhow::{Error, Ok};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags {
    line_num: bool,
    file_name_only: bool,
    case: bool,
    invert: bool,
    entire: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            line_num: flags.contains(&"-n"),
            file_name_only: flags.contains(&"-l"),
            case: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            entire: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut ans = Vec::new();
    // check for the multifile too
    for file in files{
        let content = fs::read_to_string(file)?;
        let found = search(pattern,flags,content)?;
        if flags.file_name_only && found.len() > 0{
            ans.push(file.to_string());
            continue;
        }
        match files.len() > 1 {
            true => {
                for line in found{
                    let mut s = String::from(file.to_string());
                    s.push(':');
                    s.push_str(&line);
                    ans.push(s);
                }
            }
            _ => {
                for line in found{
                    ans.push(line);
                }
            }
        }
    }
    Ok(ans)
}
fn search(pattern: &str,flags: &Flags,content : String) -> Result<Vec<String>,Error>{
    let mut ans = Vec::new();
    let mut to_find = pattern.to_string();
    if flags.case{to_find = to_find.to_lowercase();}
    let mut line_no = 0;
    for line in content.lines(){
        let mut s = String::new();
        line_no += 1;
        let mut low_line = line.to_string();
        if flags.case {low_line = line.to_lowercase();}
        match flags.invert{
            false => {
                if low_line.contains(&to_find){
                    if flags.line_num {
                        s.push_str(&line_no.to_string());
                        s.push(':');
                    }
                    if flags.entire && low_line == to_find{
                        s.push_str(line);
                        ans.push(s);
                        continue;
                    }else if flags.entire{
                        continue;
                    }
                    s.push_str(line);
                    ans.push(s);
                }
            }
            _ => {
                if !low_line.contains(&to_find){
                    if flags.line_num {
                        s.push_str(&line_no.to_string());
                        s.push(':');
                    }
                    s.push_str(line);
                    ans.push(s);
                }
            }
        }
    }
    Ok(ans)
}
