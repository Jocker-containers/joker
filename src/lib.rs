use clap::{arg, Command};


pub fn cli() -> Command {
    Command::new("joker")
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
                .arg(arg!(<CONTAINER_NAME> ... "Stuff to add")),
        )
}