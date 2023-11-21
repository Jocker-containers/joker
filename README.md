# joker

This is a CLI-part of the Joker Project. Made with ❤️ using Rust.
## Compilation
```shell
git clone git@github.com:Joker-containers/joker.git
cd joker
./compile.sh
```
## Usage

### General

```shell
A cli component of the joker project.

Usage: joker <COMMAND>

Commands:
  add       Add a new daemon with custom ip and port.
  checkout  Switch to a daemon.
  run       Run specified containers on a current daemon.
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Adding daemon

```shell
Usage: joker add [OPTIONS] <DAEMON_NAME>

Arguments:
  <DAEMON_NAME>  The name of the daemon.

Options:
  -i, --ip <IP_ADDRESS>  The ip-address of the host.
  -p, --port <PORT>      The port of the host.
  -h, --help             Print help
```

### Checkouting to daemon

```shell
Usage: joker checkout <DAEMON_NAME>

Arguments:
  <DAEMON_NAME>  The name of the daemon to checkout.

Options:
  -h, --help  Print help

```

### Running containers

```shell
Usage: joker run <CONTAINER_NAME>...

Arguments:
  <CONTAINER_NAME>...  Stuff to add

Options:
  -h, --help  Print help
```