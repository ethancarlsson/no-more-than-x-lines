use line::LineFinder;

mod command;
mod line;

fn main() {
    let line_finder = LineFinder {
        string_to_find: "\n\n\n".to_string(),
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

    println!("{}", stringed_invalids)
}
