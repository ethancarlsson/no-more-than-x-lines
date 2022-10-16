use line::LineFinder;

mod command;
mod line;
mod fix_file;

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

    let stringed_invalids = match files_with_invalid_string {
        Ok(lines_changed) => lines_changed
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let _file_fixer = fix_file::new_file_fixer(string_to_find.to_string(), "\n\n".to_string());


    println!("{}", stringed_invalids)
}
