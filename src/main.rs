mod command;

fn main() {
    match command::read_diff() {
        Ok(ok) => println!("{}", ok),
        Err(e) => println!("{}", e)
    }
}
