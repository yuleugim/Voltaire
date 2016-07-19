#[macro_use]
extern crate clap;

mod action;

use std::path::PathBuf;
use std::process::Command;
use clap::App;
use action::scan;
use action::process;

fn main() {
    let yaml = load_yaml!("commands.yml");
    let cli_input = App::from_yaml(yaml).get_matches();

    // Set the path to volatility via user input or a basic search
    let volatility_path = if let Some(path) = cli_input.value_of("path-to-volatility") {
        PathBuf::from(path)
    } else {
        fetch_volatility().unwrap()
    };

    let pair = if let Some(name) = cli_input.subcommand_name() {
        (name, cli_input.subcommand_matches(name).unwrap())
    } else {
        panic!("No subcommand provided")
    };

    //Decide which code path to execute given a subcommand
    match pair {
        ("scan", args) => scan::execute(volatility_path, args),
        ("process", args) => process::execute(volatility_path, args),
        _ => panic!()
    };
}

fn fetch_volatility() -> Option<PathBuf> {
    // If we're on Windows, we'll only check the current directory
    let search_paths = if cfg!(target_os = "windows") {
        vec![r#".\vol.exe"#, r#".\vol.py"#]
    } else {
        vec!["/usr/bin/volatility",
             "./vol",
             "/bin/vol",
             "/usr/bin/vol",
             "/usr/local/bin/vol",
             "./vol.py",
             "/bin/vol.py",
             "/usr/bin/vol.py",
             "/usr/local/bin/vol.py",
             "./volatility",
             "/bin/volatility",
             "/usr/bin/volatility",
             "/usr/local/bin/volatility",
             "./volatility.py",
             "/bin/volatility.py",
             "/usr/bin/volatility.py",
             "/usr/local/bin/volatility.py"]
    };

    for test_path in search_paths {
        let path = PathBuf::from(test_path);

        if path.exists() {
            return Some(path);
        };
        panic!("Unable to find volatility executable. Please use -p.")
    }

    None
}
