use std::fs;
use std::path::{Path, PathBuf};

pub fn execute(volatility_path: PathBuf, args: &super::ArgMatches) {
    let source = Path::new(args.value_of("source").unwrap());
    let destination = Path::new(args.value_of("destination").unwrap());
    let es = args.value_of("es").unwrap();
    let profile = args.value_of("profile").unwrap();

    if !source.exists() {
        panic!("Input file not found");
    }

    if !destination.exists() {
        println!("Creating {}", destination.to_string_lossy());
        fs::create_dir_all(&destination).unwrap();
    }

    // Run through and execute each of these volatility commands
    let tests = vec!["pslist",
                     "pstree",
                     "netscan",
                     "psxview",
                     "consoles",
                     "psscan",
                     "mutantscan -s",
                     "cmdscan",
                     "dlllist",
                     "filescan",
                     "iehistory",
                     "svcscan",
                     "modules",
                     "modscan",
                     "sessions",
                     "messagehooks",
                     "windows",
                     "wintree",
                     "clipboard",
                     "deskscan"];
    for test in tests {
        println!("Starting {}", test);

        let outfile = format!("{}/ES{}_{}.txt", destination.to_str().unwrap(), es, test);

        let result = super::Command::new(&volatility_path)
            .arg("-f")
            .arg(&source)
            .arg(format!("--profile={}", &profile))
            .arg(test)
            .arg(format!("--output-file={}", &outfile))
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                println!("{}", String::from_utf8(output.stdout).unwrap());
                println!("Successful execution of {}.", test)
            } else {
                println!("{}", String::from_utf8(output.stderr).unwrap());
                println!("Failure executing {}. Exiting.", test);
            }
        } else {
            println!("Failure executing {}. Exiting.", test);
        }
    }

    // If we're running Voltaire on Windows, we can execute another test
    // ACTUALLY we need to check if it's a Windows profile
    if cfg!(target_os = "windows") {
        let outfile = format!("{}ES{}_autorun.txt", &destination.to_str().unwrap(), es);

        let result = super::Command::new(volatility_path)
            .arg("-f")
            .arg(source)
            .arg(format!("--profile={}", profile))
            .arg("printkey")
            .arg(r#""Software\Microsoft\Windows\CurrentVersion\Run\""#)
            .arg(format!("--output-file={}", outfile))
            .output();

        if let Ok(output) = result {
            if output.status.success() {
                println!("{}", String::from_utf8(output.stdout).unwrap());
                println!("Successful execution of autorun");
            } else {
                println!("{}", String::from_utf8(output.stderr).unwrap());
                println!("Failure executing autorun. Exiting.");
            }
        } else {
            println!("Failure executing autorun. Exiting.");
        }
    }
}
