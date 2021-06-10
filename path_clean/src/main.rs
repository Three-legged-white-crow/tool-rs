extern crate clap;
use clap::App;
use clap::Arg;

fn main() {
    let flags = App::new("Path Clean")
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
    let mut clean_path = matches.value_of("path").unwrap_or("");

    match clean_path {
        "" => {
            println!();
            println!("[Error] must specify directiory that wait clean!");
            println!();
            flags_backup.print_long_help();
            return;
        },

        _ => clean_path = clean_path.trim_end_matches('/')
    }

    let num_verbose = matches.occurrences_of("verbose");
    let mut show_detail: bool = false;

    if num_verbose > 0 {
        show_detail = true
    }

    clean(clean_path, show_detail);

}

// todo: clean file
fn clean(clean_path: &str, show_detail: bool) {
    println!("cleaning path: {}, show detail: {}", clean_path, show_detail)
}