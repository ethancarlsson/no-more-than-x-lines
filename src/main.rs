use line::LineFinder;

mod command;
mod fix_file;
mod line;

fn main() {
    let string_to_find = "\n\n\n";

    let line_finder = LineFinder {
        string_to_find: string_to_find.to_string(),
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
                fix_file::new_file_fixer(string_to_find.to_string(), "\n\n".to_string());

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
