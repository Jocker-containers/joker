pub mod errors;
pub mod container;
pub mod daemon;


use std::io;
use std::io::{Read, Write};
use clap::{arg, Command};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use crate::daemon::{Daemon, get_config, write_config, Requests};
use crate::errors::AbsentHashMapKeyError;

/// The function to get the help message.
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
                .about("Run specified containers on a current daemon.")
                .arg_required_else_help(true)
                .arg(arg!(<CONTAINER_NAME> ... "Containers to run"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("trace")
                .about("Trace the events on the daemon. Uses stdout by default.")
        )
        .subcommand(
            Command::new("logs")
                .about("Get the output of the specified container.")
                .arg(arg!(<CONTAINER_NAME> "The name of the container to get logs from. \
                Uses stdout by default"))
                .arg_required_else_help(true),
        )
}

/// Entry function which executes cli commands.
/// It parses the command and its arguments and then calls a
/// corresponding Rust function.
pub fn execute(command: &mut Command) -> Result<(), Box<dyn std::error::Error>> {
    let matches = command.clone().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let daemon_name = sub_matches.get_one::<String>("DAEMON_NAME").expect("Daemon name is required, but not provided.");
            let ip_addr = sub_matches.get_one::<String>("ip").expect("IP address is required, but not provided.");
            let port = sub_matches.get_one::<String>("port").expect("Port number is required, but not provided.");

            match add_daemon(daemon_name, ip_addr, port) {
                Ok(_) => {
                    Ok(())
                }
                Err(err) => {
                    println!("Error while adding daemon: {}", err);
                    Err(err)
                }
            }
        }
        Some(("checkout", sub_matches)) => {
            let daemon_name = sub_matches.get_one::<String>("DAEMON_NAME").expect("required");

            checkout_daemon(daemon_name)
        }
        Some(("run", sub_matches)) => {
            let containers = sub_matches
                .get_many::<String>("CONTAINER_NAME")
                .into_iter()
                .flatten()
                .map(|x| x.as_str())
                .collect::<Vec<_>>();

            run_containers(&containers)
        }
        Some(("trace", _)) => {
            daemon_trace()
        }
        Some(("logs", sub_matches)) => {
            let container = sub_matches.get_one::<String>("CONTAINER_NAME")
                .ok_or("Container name should be provided")?;
            get_logs(container)
        }
        _ => {
            println!("Error: no such subcommand.");
            show_help_message(command)
        },
    }
}

/// Adds a daemon with specified ip address and port.
/// Propagates the error down the stack trace.
fn add_daemon(daemon_name: &str, ip_addr: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: add checking whether we modify current daemon
    let mut config = daemon::get_config()?;

    let socket_addr = SocketAddr::new(IpAddr::from_str(ip_addr)?, port.parse()?);

    config.daemons.insert(daemon_name.to_owned(), socket_addr);

    println!(
        "Added daemon {} at ip {} and port {}.",
        daemon_name,
        ip_addr,
        port,
    );

    write_config(&config)?;

    Ok(())
}

/// Changes current daemon to a specified one.
/// Propagates the error down the stack trace.
fn checkout_daemon(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = daemon::get_config()?;

    match config.daemons.get(name) {
        None => {
            println!(
                "Error while switching to daemon {}: no such daemon.",
                name,
            );

            Err(Box::new(AbsentHashMapKeyError))
        }
        Some(&socket_address) => {
            let name = name.to_owned();

            println!(
                "Switching to daemon {}.",
                name,
            );

            let previous = config.current_daemon.clone();
            config.current_daemon = Daemon {name, socket_address};
            config.daemons.insert(previous.name, previous.socket_address);

            write_config(&config)?;

            Ok(())
        }
    }
}

/// Sends containers to current daemon.
/// Propagates the error down the stack trace.
fn run_containers(containers: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config()?;

    let mut tcp_stream = TcpStream::connect(config.current_daemon.socket_address)?;
    tcp_stream.set_nodelay(true)?;

    println!("Connection established. Sending files to a daemon");

    for &container_path in containers {

        let binary_name = container_path.split('/').last()
            .ok_or("Error: bad file path.")?.as_bytes().to_owned();
        let binary = std::fs::read(container_path)?;
        let binary_config = std::fs::read(format!("{}.joker", container_path))?;

        // Send the type of request
        let request = Requests::Run;
        tcp_stream.write_all(&[request as u8])?;

        // Send the size of binary name and binary name itself
        tcp_stream.write_all(&(binary_name.len() as u64).to_le_bytes())?;
        tcp_stream.write_all(&binary_name)?;

        // Send the size of the binary and the binary itself
        tcp_stream.write_all(&(binary.len() as u64).to_le_bytes())?;
        tcp_stream.write_all(&binary)?;

        // Send the size of binary config and binary config itself
        tcp_stream.write_all(&(binary_config.len() as u64).to_le_bytes())?;
        tcp_stream.write_all(&binary_config)?;
    }

    println!(
        "Running containers {} at daemon {}.",
        containers.join(", "),
        "current daemon".to_owned(),
    );

    Ok(())
}

/// Prints daemon messages to a standard output.
/// Propagates the error down the stack trace.
fn daemon_trace() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config()?;

    let mut tcp_stream = TcpStream::connect(config.current_daemon.socket_address)?;
    tcp_stream.set_nodelay(true)?;

    // writing a request to a daemon
    let request = Requests::Trace;
    tcp_stream.write_all(&[request as u8])?;

    // getting result from a daemon
    let received_data = String::from_utf8(read_all_from_stream(&mut tcp_stream)?)?;

    println!("{}", received_data);

    Ok(())
}

/// Receives a log of a specified container.
/// Propagates the error down the stack trace.
fn get_logs(container_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config()?;

    let mut tcp_stream = TcpStream::connect(config.current_daemon.socket_address)?;
    tcp_stream.set_nodelay(true)?;

    // writing request to a daemon
    let request = Requests::Logs;
    tcp_stream.write_all(&[request as u8])?;
    // writing container name to a daemon
    tcp_stream.write_all(&container_name.len().to_le_bytes())?;
    tcp_stream.write_all(container_name.as_bytes())?;

    // getting result from a daemon
    let received_data = String::from_utf8(read_all_from_stream(&mut tcp_stream)?)?;

    println!("{}", received_data);

    Ok(())
}

/// Shows help message.
fn show_help_message(command: &mut Command) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", command.render_help());
    Ok(())
}

fn read_all_from_stream(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut size_of_message = [0u8; 8];
    stream.read_exact(&mut size_of_message[..])?;
    let size_of_message = u64::from_be_bytes(size_of_message);

    let mut message = vec![0; size_of_message as usize];
    stream.read_exact(&mut message[..])?;

    Ok(message)
}