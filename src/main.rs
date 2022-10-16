use std::env;

use line::LineFinder;
use regex::Regex;

mod command;
mod fix_file;
mod line;

fn main() {
    let allowed_consecutive_lines = get_allowed_lines();

    let string_to_find = format!("{}{}", allowed_consecutive_lines, "\n");

    let line_finder = LineFinder {
        string_to_find: string_to_find.clone(),
    };

    let files_with_invalid_string = match command::fetch_diff() {
        Ok(ok) => line_finder.get_changed_lines_per_file(ok),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match files_with_invalid_string {
        Ok(lines_changed) => {
            let file_fixer =
                fix_file::new_file_fixer(string_to_find, allowed_consecutive_lines);

            for invalid_file in lines_changed {
                file_fixer.replace_unwanted_strings(invalid_file);
            }
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

fn get_allowed_lines() -> String {
    let args: Vec<String> = env::args().collect();

    let line_num_opt = Regex::new(r"--lines=\d+").unwrap();
    let num_string = args.iter()
        .find(|arg| line_num_opt.is_match(arg));

    let allowed_new_lines = match num_string {
        Some(num_str) => {
            num_str.replace("--lines=", "").parse::<usize>()
        },
        None => Ok(2)
    };

    let n = match allowed_new_lines {
        Ok(num) => num,
        Err(_) => 2
    };

    vec!["\n"; n].join("")
}
