# tuli

<p>
    <a href="https://crates.io/crates/tuli" target="_blank">
        <img alt="Version" src="https://img.shields.io/crates/v/tuli" />
   </a>
</p>

An experimental local server for serving static files that sort of works (Do not use, very experimental)

## Usage

Install (requires cargo):

```shell
cargo install tuli
```

Basic usage:

```
tuli serve --port 8000 --dir ./public
```

Tuli will serve the files in the `public` directory provided it has an `index.html` file. Images are broken at the moment though, so only UTF-8/ application/text content can be served with the current implementation.

### Commands

`tuli --help`

```shell
tuli 0.1.0
Collins Muriuki <murerwacollins@gmail.com>


USAGE:
    tuli [SUBCOMMAND]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    serve    Serves static files in a given directory
```

`tuli serve --help`

```
tuli-serve
Serves static files in a given directory

USAGE:
    tuli serve [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <dir>      The public directory with static files that should be served, defaults to the current directory
    -p, --port <port>    The port the server should run on, default is 8080
```
