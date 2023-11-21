cargo build --release
echo '{"current_daemon":{"name":"localhost","socket_address":"127.0.0.1:8080"},"daemons":{}}' > config.cfg
cargo install --path .