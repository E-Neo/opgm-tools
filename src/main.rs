use clap::{load_yaml, App, AppSettings, ArgMatches};
use opgm_tools::data_graph::{bin_to_sqlite3, read_edges_file};
use std::{error::Error, fs::File};

fn handle_data(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
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
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("data") {
        handle_data(matches)?;
    }
    Ok(())
}
