use clap::{App, Arg};
use std::fs::{create_dir, write, File};
use std::io::Error;
use std::path::Path;

pub fn run() -> Result<(), Error> {
    let matches = App::new("kewp")
        .version("1.0")
        .author("Your Name <yourname@example.com>")
        .about("A CLI tool to create a new app directory with static, src, and out folders and files")
        .arg(
            Arg::with_name("appname")
                .help("Sets the name of the new app directory")
                .required(true),
        )
        .arg(
            Arg::with_name("create")
                .help("Creates the new app directory with the necessary files and folders"),
        )
        .get_matches();

    let appname = matches.value_of("appname").unwrap();
    if matches.is_present("create") {
        create_app_directory(appname)?;
    } else {
        println!("Usage: kewp <appname> create");
    }

    Ok(())
}

fn create_app_directory(appname: &str) -> Result<(), Error> {
    create_dir(appname)?;
    create_dir(format!("{}/static", appname))?;
    create_dir(format!("{}/src", appname))?;
    create_dir(format!("{}/out", appname))?;

    let main_qep = format!("{}/src/main.qep", appname);
    let overview_qep = format!("{}/src/overview.qep", appname);
    let connect_apl = format!("{}/src/connect.apl", appname);
    let kewp_toml = format!("{}/kewp.toml", appname);
    let gitignore = format!("{}/.gitignore", appname);

    File::create(&main_qep)?;
    File::create(&overview_qep)?;
    File::create(&connect_apl)?;

    write(
        Path::new(&kewp_toml),
        format!(
            r#"{}
name = "{}"
version = "0.1.0"

[dependencies]"#,
            "[package]",
            appname
        ),
    )?;

    write(Path::new(&gitignore), ".DS_Store")?;

    write(
        Path::new(&main_qep),
        r#"import pkg standard::all

func main(args, pkg) {
    fn.print("Hello, World");
}"#,
    )?;

    println!("Created app directory: {}", appname);

    Ok(())
}
