use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("joker")
        .arg_required_else_help(true)
        .about("A cli component of the joker project.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("add")
                .about("Add a new daemon with custom ip and port.")
                .arg(arg!(<DAEMON_NAME> "The name of the daemon."))
                .arg_required_else_help(true)
                .arg(arg!(-i --ip <IP_ADDRESS> "The ip-address of the host."))
                .arg_required_else_help(true)
                .arg(arg!(-p --port <PORT> "The port of the host."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("checkout")
                .about("Switch to a daemon.")
                .arg(arg!(<DAEMON_NAME> "The name of the daemon to checkout."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("run")
                .about("Runs specified containers on a current daemon.")
                .arg_required_else_help(true)
                .arg(arg!(<CONTAINER_NAME> ... "Stuff to add"))
                .arg_required_else_help(true),
        )
}

pub fn execute(command: &mut Command) {
    let matches = command.clone().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let daemon_name = sub_matches.get_one::<String>("DAEMON_NAME").expect("required");
            let ip_addr = sub_matches.get_one::<String>("ip").expect("required");
            let port = sub_matches.get_one::<String>("port").expect("required");

            add_daemon(daemon_name, ip_addr, port);
        }
        Some(("checkout", sub_matches)) => {
            let daemon_name = sub_matches.get_one::<String>("DAEMON_NAME").expect("required");

            checkout_daemon(daemon_name);
        }
        Some(("run", sub_matches)) => {
            let containers = sub_matches
                .get_many::<String>("CONTAINER_NAME")
                .into_iter()
                .flatten()
                .map(|x| x.as_str())
                .collect::<Vec<_>>();

            run_container(&containers);
        }
        _ => show_help_message(command),
    }
}

fn add_daemon(daemon_name: &str, ip_addr: &str, port: &str) {
    println!(
        "Creating daemon {} at ip {} and port {}.",
        daemon_name,
        ip_addr,
        port,
    );
}

fn checkout_daemon(daemon_name: &str) {
    println!(
        "Switching to daemon {}.",
        daemon_name,
    );
}

fn run_container(containers: &[&str]) {
    println!(
        "Running containers {} at daemon {}.",
        containers.join(", "),
        "current daemon".to_owned(),
    );
}

fn show_help_message(command: &mut Command) {
    command.print_help().unwrap();
}
