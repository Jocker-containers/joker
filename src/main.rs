use joker::*;

fn main() {
    let mut command = cli();
    match execute(&mut command) {
        Ok(_) => {},
        Err(err) => {println!("Execution was stopped because of the previous error: {}", err)},
    }
}
