#[derive(Debug, PartialEq)]
pub struct LinesChanged {
    file: String,
    from: u32,
    to: u32,
}

impl LinesChanged {
    pub fn to_string(&self) -> String {
        format!(
            "{} {}-{}",
            self.file,
            self.from.to_string(),
            self.to.to_string()
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

    advance_iter_by(&mut split_string, 3);

    let line = split_string.next();
    let (from, to) = match get_from_to(line) {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    Ok(LinesChanged {
        file: file_name,
        from,
        to,
    })
}

fn get_from_to(line: Option<&str>) -> Result<(u32, u32), String> {
    let (from, to) = match line {
        Some(l) => {
            let sub_l = match get_changed_line_numbers(l) {
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

            (from_n, to_n)
        }
        None => (0, 0),
    };
    Ok((from, to))
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

fn advance_iter_by(split_string: &mut std::str::Split<&str>, n: u32) {
    for _ in 0..n {
        split_string.next();
    }
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
            from: 7,
            to: 30,
        };

        assert_eq!(lines_changed_from_diff(input).unwrap(), expected);
    }
}
