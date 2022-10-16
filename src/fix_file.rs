use std::fs;

use crate::line::LinesChanged;

pub struct FileFixer {
    string_to_fix: String,
    preferred_string: String,
}

pub fn new_file_fixer(string_to_fix: String, preferred_string: String) -> FileFixer {
    FileFixer {
        string_to_fix,
        preferred_string,
    }
}

impl FileFixer {
    pub fn replace_unwanted_strings(&self, changed_lines: LinesChanged) {
        let contents =
            fs::read_to_string(changed_lines.get_file_path()).expect("unable to read file at");

        let fixed_src = self.fix_src(contents, changed_lines.get_lines());

        match fs::write("testing.rs", fixed_src) {
            Ok(_) => println!("fixed file, {}", changed_lines.get_file_path()),
            Err(e) => println!("file fixing failed\n{}", e),
        };
    }

    pub fn fix_src(&self, contents: String, changed_lines: Vec<(u32, u32)>) -> String {
        let split_file = contents.split("\n").collect::<Vec<&str>>();
        let mut chunked_file: Vec<String> = Vec::new();

        // let line_1 = split_file[0..3].join("\n");
        let mut prev_end = 0;

        for line in changed_lines {
            println!("changing ({}, {})", line.0, line.1);
            let mut start = line.0 as usize - 1;
            let end = start + line.1 as usize;

            let mut untouched_lines = "".to_string();

            if prev_end < start {
                println!("prev: {}, start: {}\n", prev_end, start);
                untouched_lines = split_file[prev_end..start].join("\n");
                start+=1
            }

            let mut fixed_lines = split_file[start..end]
                .join("\n");

            if fixed_lines.contains(&self.string_to_fix) {
                fixed_lines = fixed_lines.replace(&self.string_to_fix, &self.preferred_string)
            }

            if untouched_lines.len() > 0 {
                chunked_file.push(untouched_lines + "\n" + &fixed_lines);
            } else {
                chunked_file.push(fixed_lines)
            }

            prev_end = end + 1
        }

        chunked_file.join("\n")
    }
}












#[cfg(test)]
mod tests {
    use crate::fix_file::FileFixer;

    #[test]
    fn test_file_fixer() {
        let input_src = r#" 

vim.cmd("set nowrap")


"#
        .to_string();

        let input_lines_to_change: Vec<(u32, u32)> = vec![(1, 5)];

        let file_fixer = FileFixer {
            string_to_fix: "\n\n\n".to_string(),
            preferred_string: "\n\n".to_string(),
        };

        let expected = r#"

vim.cmd("set nowrap")

"#
        .to_string();

        assert_eq!(
            file_fixer.fix_src(input_src, input_lines_to_change),
            expected
        );
    }
}
