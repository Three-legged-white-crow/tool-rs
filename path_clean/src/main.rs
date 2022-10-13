extern crate clap;
use clap::App;
use clap::Arg;

use std::time::SystemTime;
use std::time::Duration;
use std::path::Path;
use std::io::Error;
use std::fs;


fn main() {
    let flags = App::new("path_clean")
        .version("v1.0")
        .about("Delete files recursively while preserving the directory structure")

        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("aim dir")
            .help("directory that wait clean")
            .takes_value(true))

        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("show detail of clean"));

    let mut flags_backup = flags.clone();
    let matches = flags.get_matches();
    let mut clean_path_str = matches.value_of("path").unwrap_or("");

    match clean_path_str {
        "" => {
            println!();
            println!("[Error] must specify directory that wait clean!");
            println!();
            flags_backup.print_long_help();
            return;
        },

        _ => clean_path_str = clean_path_str.trim_end_matches('/')
    }

    let num_verbose = matches.occurrences_of("verbose");
    let mut show_detail: bool = false;

    if num_verbose > 0 {
        show_detail = true
    }

    let clean_path = Path::new(clean_path_str);

    let start_time = SystemTime::now();

    let clean_res = clean(clean_path, &show_detail);
    match clean_res {
        Ok(msg) => {println!("{}", msg)}
        Err(err) => {println!("get err when clean: {}", err)}
    }

    let end_time = SystemTime::now();

    match end_time.duration_since(start_time) {
        Ok(use_time) => {
            println!("time use: {} ns", time_handle(use_time))
        }

        Err(err) => {
            println!("time use: -");
            println!("occur err: {} when get time use", err)
        }
    }


}

fn time_handle(time_use: Duration) -> String {
    let nano_time = time_use.as_nanos();
    return nano_time.to_string();
}

// todo: clean file
fn clean(clean_path: &Path, show_detail: &bool) -> Result<&'static str, Error> {

    if !clean_path.is_dir() {
        println!("clean aim: {} is file, direct remove", clean_path.to_str().unwrap());
        match fs::remove_file(clean_path) {
            Err(e) => {
                println!("[Remove File]Failed to remove file: {}, and err: {}", clean_path.to_str().unwrap(), e);
                return Err(e);
            }

            Ok(_) => {
                println!("[Remove File]Succeed to remove file: {}", clean_path.to_str().unwrap());
                return Ok("clean complete...");
            }
        }
    }

    println!("clean aim: {} is directory, recursive remove file", clean_path.to_str().unwrap());
    for entry in clean_path.read_dir()? {
        match entry {

            Err(e) => {
                // todo: add to failed list
                println!("[Clean Dir]Occur err: {}, when read dir: {}", e, clean_path.to_str().unwrap());
                continue
            }

            Ok(dir) => {
                let dir_path = dir.path();

                if dir_path.is_dir() {
                    clean(dir_path.as_path(), show_detail);
                    continue
                }

                let file_name = dir_path.to_str().unwrap_or("");
                if file_name.len() == 0 {
                    continue
                }

                println!("[Clean Dir]Need rm file, path:{}", file_name);
               
                match fs::remove_file(file_name) {

                    Err(e) => {
                        // todo: add dir to failed list
                        if *show_detail {
                            println!("[Remove File]Failed to remove file: {}, and err: {}", file_name, e)
                        }

                    }

                    Ok(_) => {
                        if *show_detail {
                            println!("[Remove File]Succeed to remove file: {}", file_name)
                        }
                    }

                }
            }
        }
    }


    return Ok("clean complete...");

}