# Demo YAML to JSON and JSON to YAML command line utility

In this demo I just wanted to show that developing CLIs in Rust is a breeze.

There are three versions of this demo stored in separate branches:

* [`main`](https://github.com/frol/clap-yaml-json-demo/tree/main) - lazy and short version without proper error handling
* [`with-error-handling`](https://github.com/frol/clap-yaml-json-demo/tree/with-error-handling) - added the error handling
* [`with-custom-type`](https://github.com/frol/clap-yaml-json-demo/tree/with-custom-type) - added a custom type `ExistingPath` to showcase that you can validate anything at CLI arguments parsing

[![asciicast](https://asciinema.org/a/GHMTURjyGxh3AqN1V4bUYj8ht.svg)](https://asciinema.org/a/GHMTURjyGxh3AqN1V4bUYj8ht)

## Build

You will need [Rust](https://www.rust-lang.org/) installed on your computer, and then run:

```sh
cargo build --release
```

## Run

After the build, the executable file is usually located at `target/release/clap-yaml-json-demo` (or with `.exe` extension on Windows), so run it:

On Linux/MacOS:

```
./target/release/clap-yaml-json-demo
```

On Windows:

```
.\target\release\clap-yaml-json-demo.exe
```

You will be greeted with clap help message that is generated automatically from the code!
All the CLI arguments are validated and once you get the `Command` type, you can trust that all the types are valid (e.g. if you expect a number, it will be a number, not a string that you will still need to parse).
