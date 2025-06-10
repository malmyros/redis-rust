# Redis in Rust

I am building my own simple Redis server to learn rust
and also understand the low level intricacies of building
distributed systems.

## Building a running the server

To build the server with cargo by running the following command:

```
cargo build
```

Once the build has been created we can run the server using:

```
cargo run
```

This will run the build from the target folder automatically.

To check if everything compiles we can run

```
cargo check
```


## Install redis-cli

To test the client we need to first install redis-cli.
We can do this easily with homebrew by running:

```
brew install redis
```

This will install both redis and redis-cli.

To verify the cli has been installed we can run:

```
redis-cli --version
```

We should see a response like the following depending on
the version you have installed.

```
redis-cli 8.0.2
```

## Send a Command
```
echo -e "ping\nping\nping" | redis-cli
```