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

        let mut prev_end = 0;

        for line in changed_lines {
            let start = (line.0 ) as usize;
            let end = start + (line.1 ) as usize;
            println!("matching {}..{}..{}\n", prev_end, start, end);

            let mut untouched_lines = "".to_string();

            if prev_end < start {
                untouched_lines = split_file[prev_end..start].join("\n");
            }

            let mut fixed_lines = split_file[start..end].join("\n");

            fixed_lines = self.remove_unwanted_string(fixed_lines);

            if untouched_lines.len() > 0 {
                chunked_file.push(untouched_lines + "\n" + &fixed_lines);
            } else {
                chunked_file.push(fixed_lines)
            }

            prev_end = end
        }

        if prev_end != split_file.len() {
            chunked_file.push(split_file[prev_end..split_file.len()].join("\n"));
        }

        chunked_file.join("\n")
    }

    fn remove_unwanted_string(&self, fixed_lines: String) -> String {
        if !fixed_lines.contains(&self.string_to_fix) {
            return fixed_lines;
        }
        // keep looking in case the first time creates another case
        // i.e. remove "\n\n" and replace with \n in "\n\n\n\n" ->  "\n\n" (need to recurse here)
        return self.remove_unwanted_string(
            fixed_lines.replace(&self.string_to_fix, &self.preferred_string),
        );
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
