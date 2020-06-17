# doit
[![Build Status](https://h2869596.stratoserver.net/api/badges/flovanco/doit/status.svg)](https://h2869596.stratoserver.net/flovanco/doit)
[![GPL 3 licensed](https://img.shields.io/badge/license-GPL3-darkgreen.svg)](./LICENSE)

Task app based on OrbTk https://github.com/redox-os/orbtk.

<img alt="Space Editor" width="420" src="https://codeberg.org/flovanco/assets/raw/branch/master/doit.png">

## Live Demo

https://pages.codeberg.org/flovanco/apps/doit/index.html


## Platforms

* Redox OS (native)
* Linux (native | cargo-node wip)
* macOS (native | cargo-node wip)
* Windows (native | cargo-node wip)
* openBSD (not tested, but should work)
* Web

## Run 

You can start the editor by executing the following command:

```text
cargo run --release
```

## Run with cargo-node

To run the editor on as browser or electron app you have to install

```text
cargo install -f cargo-node
```

Before you could use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You could download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatic.

### Start 

* Run as browser app:

```text
cargo node run --browser
```

* Run as electron app:

```text
cargo node run --electron
```

## License

Licensed under GPL v3 license ([LICENSE](LICENSE))