use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};
use opgm_tools::{
    data_graph::{bin_to_sqlite3, snap_edges_to_bin, sqlite3_to_graphflow},
    pattern_graph::{gisp_to_graphflow, parse},
};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

fn handle_createdb(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.value_of("FMT").unwrap() {
        "snap_edges" => {
            File::create(matches.value_of("SQLITE3").unwrap())?;
            let (vertices, edges) = snap_edges_to_bin(matches.value_of("INPUT").unwrap())?;
            let conn = sqlite::open(matches.value_of("SQLITE3").unwrap())?;
            bin_to_sqlite3(&conn, &vertices, &edges)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn handle_convertdb(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.value_of("FMT").unwrap() {
        "graphflow" => {
            let conn = sqlite::open(matches.value_of("SQLITE3").unwrap())?;
            let output = Path::new(matches.value_of("OUTPUT").unwrap());
            let path = output.parent().expect("OUTPUT path");
            let name = output
                .file_name()
                .expect("OUTPUT name")
                .to_str()
                .expect("OUTPUT UTF-8 name");
            let mut vertices_buf =
                BufWriter::new(File::create(path.join(format!("{}_vertices.csv", name)))?);
            let mut edges_buf =
                BufWriter::new(File::create(path.join(format!("{}_edges.csv", name)))?);
            sqlite3_to_graphflow(&conn, &mut vertices_buf, &mut edges_buf)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn handle_convertgisp(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.value_of("FMT").unwrap() {
        "graphflow" => {
            let mut gisp = String::new();
            BufReader::new(File::open(matches.value_of("GISP").unwrap())?)
                .read_to_string(&mut gisp)?;
            writeln!(
                &mut BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?),
                "{}",
                gisp_to_graphflow(&parse(&gisp)?)
            )?;
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
                        .possible_values(&["snap_edges"]),
                )
                .arg(Arg::with_name("INPUT").required(true))
                .arg(Arg::with_name("SQLITE3").required(true)),
        )
        .subcommand(
            SubCommand::with_name("convertdb")
                .about("Converts SQLite3 file to other format")
                .arg(
                    Arg::with_name("FMT")
                        .required(true)
                        .possible_values(&["graphflow"]),
                )
                .arg(Arg::with_name("SQLITE3").required(true))
                .arg(Arg::with_name("OUTPUT").required(true)),
        )
        .subcommand(
            SubCommand::with_name("convertgisp")
                .about("Converts gisp file to other format")
                .arg(
                    Arg::with_name("FMT")
                        .required(true)
                        .possible_values(&["graphflow"]),
                )
                .arg(Arg::with_name("GISP").required(true))
                .arg(Arg::with_name("OUTPUT").required(true)),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("createdb") {
        handle_createdb(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("convertdb") {
        handle_convertdb(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("convertgisp") {
        handle_convertgisp(matches)?;
    }
    Ok(())
}
