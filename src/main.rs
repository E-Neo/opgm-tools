use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};
use opgm_tools::data_graph::{bin_to_sqlite3, read_edges_file};
use std::{error::Error, fs::File};

fn handle_createdb(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.value_of("FMT").unwrap() {
        "snap_edges" => {
            File::create(matches.value_of("OUTPUT").unwrap())?.set_len(0)?;
            let (vertices, edges) = read_edges_file(matches.value_of("INPUT").unwrap())?;
            let conn = sqlite::open(matches.value_of("OUTPUT").unwrap())?;
            bin_to_sqlite3(&conn, &vertices, &edges)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("createdb")
                .about("Creates SQLite3 file from other formats")
                .arg(
                    Arg::with_name("FMT")
                        .required(true)
                        .help("Format of input file(s)")
                        .possible_values(&["snap_edges"]),
                )
                .arg(Arg::with_name("INPUT").required(true))
                .arg(Arg::with_name("OUTPUT").required(true)),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("createdb") {
        handle_createdb(matches)?;
    }
    Ok(())
}
