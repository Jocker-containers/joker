use joker::*;

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            println!(
                "Creating daemon {} at ip {} and port {}.",
                sub_matches.get_one::<String>("DAEMON_NAME").expect("required"),
                sub_matches.get_one::<String>("ip").expect("required"),
                sub_matches.get_one::<String>("port").expect("required"),
            );
        }
        Some(("checkout", sub_matches)) => {
            println!(
                "Switching to daemon {}.",
                sub_matches.get_one::<String>("DAEMON_NAME").expect("required"),
            );
        }
        Some(("run", sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("CONTAINER_NAME")
                .into_iter()
                .flatten()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            println!(
                "Running containers {} at daemon {}.",
                args,
                "current daemon".to_owned(),
            );
        }
        _ => unreachable!(),
    }
}