use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct LinesChanged {
    file: String,
    ln_num_range: Vec<(u32, u32)>,
}

impl LinesChanged {

    pub fn get_file_path(&self) -> String {
        self.file.clone()
    }

    pub fn get_lines(&self) -> Vec<(u32, u32)> {
        self.ln_num_range.clone()
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}─────┐\n\t\t|{}",
            self.file,
            self.ln_num_range
                .iter()
                .map(|(from, to)| { return format!("{}-{}", from, to) })
                .collect::<Vec<String>>()
                .join("\n\t\t|"),
        )
    }
}

pub struct LineFinder {
    pub string_to_find: String,
}

impl LineFinder {
    pub fn get_changed_lines_per_file(&self, diff: String) -> Result<Vec<LinesChanged>, String> {
        self.get_files_with_extra_lines(diff)
    }

    fn get_files_with_extra_lines(&self, diff: String) -> Result<Vec<LinesChanged>, String> {
        let mut files_changed = Vec::new();
        for s in diff.split("diff --git ") {
            if s.contains(&self.string_to_find) {
                match lines_changed_from_diff(s) {
                    Ok(lines) => files_changed.push(lines),
                    Err(e) => return Err(e),
                };
            }
        }
        return Ok(files_changed);
    }
}

fn lines_changed_from_diff(s: &str) -> Result<LinesChanged, String> {
    let mut split_string = s.split("\n");

    let file_name_line = split_string.next();
    let file_name = match get_file_name(file_name_line) {
        Ok(value) => value,
        Err(value) => return Err(value.to_string()),
    };

    let mut from_to_vec = Vec::new();

    let re = Regex::new(r"@@ \-\d+,\d+ \+\d+,\d+ @@").unwrap();

    for line in split_string {
        if !re.is_match(line) {
            continue;
        }

        let from_to = match get_from_to(line) {
            Ok(value) => value,
            Err(value) => return Err(value),
        };
        from_to_vec.push(from_to)
    }

    Ok(LinesChanged {
        file: file_name,
        ln_num_range: from_to_vec,
    })
}

fn get_from_to(line: &str) -> Result<(u32, u32), String> {
    let sub_l = match get_changed_line_numbers(line) {
        Ok(value) => value,
        Err(value) => return Err(value),
    };
    let mut split_line_nums = sub_l.split(",");

    let from_str = split_line_nums.next().or(Some("0"));
    let to_str = split_line_nums.next().or(Some("0"));

    let from = from_str.unwrap().trim().parse::<u32>();
    let to = to_str.unwrap().trim().parse::<u32>();

    let from_n = match from {
        Ok(k) => k,
        Err(e) => return Err(e.to_string()),
    };

    let to_n = match to {
        Ok(k) => k,
        Err(e) => return Err(e.to_string()),
    };

    Ok((from_n, to_n))
}

// harduken
fn get_changed_line_numbers(l: &str) -> Result<String, String> {
    let plus_to_endln = match l.find("+") {
        Some(i) => l.get(i + 1..),
        None => return Err(format!("couldn't find the changed file, {}", l)),
    };

    match plus_to_endln {
        Some(pl) => {
            match pl.find("@") {
                Some(pl_i) => match pl.get(..pl_i) {
                    Some(str_changed_lines) => return Ok(str_changed_lines.to_string()),
                    None => return Err("couldn't get changed lines".to_string()),
                },
                None => return Err("missing @ sign".to_string()),
            };
        }
        None => return Err(format!("no changed lines in changed file {}", l)),
    };
}

fn get_file_name(file_name_line: Option<&str>) -> Result<String, &str> {
    match file_name_line {
        Some(line) => extract_filename_from_line(line),
        None => Err("Couldn't find the name of the changed file"),
    }
}

fn extract_filename_from_line(line: &str) -> Result<String, &str> {
    let n_line = line.trim_start().to_string();
    let first_space = n_line.find(" ");
    let file_name = n_line.get(2..first_space.unwrap());
    match file_name {
        Some(name) => Ok(name.to_string()),
        None => Err("couldn't find name in line"),
    }
}

#[cfg(test)]
mod tests {
    use crate::line::{lines_changed_from_diff, LinesChanged};

    #[test]
    fn test_lines_changed_from_diff() {
        let input = r#"a/init.lua b/init.lua
index c318408..aa492cf 100644
--- a/init.lua
+++ b/init.lua
@@ -7,28 +7,30 @@ vim.g.mapleader = " "
-- wrap
vim.cmd("set nowrap")

-- tabs
vim.cmd("set tabstop=8")
vim.cmd("set softtabstop=0 noexpandtab")
vim.cmd("set shiftwidth=8")
"#;
        let expected = LinesChanged {
            file: "init.lua".to_string(),
            ln_num_range: [(7, 30)].to_vec(),
        };


        assert_eq!(lines_changed_from_diff(input).unwrap(), expected);
    }
}
