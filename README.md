# Kewp

Kewp is a command-line interface (CLI) tool that creates a new app directory with static, src, and out folders and files. It is written in Rust and uses the Clap library to parse command-line arguments.

## Installation

To use Kewp, you first need to clone the repository and build the binary using Cargo:

```cmd/bash
git clone https://github.com/voltage-developer/kewp.git
cd kewp
cargo build --release
```

You can then install kewp globally using the command:

```bash/cmd
cargo install --path .
```

This will install the `kewp` binary in your system's binary directory, which should be included in your `PATH`.

## Usage

To create a new app directory with Kewp, run the command:

```cmd/bash
kewp <appname> create
```

Replace `<appname>` with the name you want to give your new app directory. This will create a new directory with the following structure:

```Files
<appname>/
  ├── static/
  ├── src/
  │   ├── main.qep
  │   ├── overview.qep
  │   └── connect.apl
  ├── out/
  ├── kewp.toml
  └── .gitignore
```
The `static` folder is for storing static assets, such as images or stylesheets. The `src` folder is for storing the source code of your application, and contains three files: `main.qep`, `overview.qep`, and `connect.apl`. The `out` folder is where the output of your application will be stored. The `kewp.toml` file contains metadata about your application, such as its name and version, as well as a list of its dependencies. The `.gitignore` file specifies files and directories that should be ignored by Git.

## License

Kewp is licensed under the GPL-3.0 License. See the LICENSE file for more information.
