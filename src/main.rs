use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};
use derive_more::{Display, Error};
use opgm_tools::{
    data_graph::{
        bin_to_sqlite3, snap_edges_to_bin, sqlite3_to_graphflow, sqlite3_to_neo4j,
        sqlite3_to_sqlite3,
    },
    pattern_graph::{gisp_to_cypher, gisp_to_graphflow, parse},
};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

#[derive(Debug, Display, Error)]
struct InvalidPath;

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
    let conn = sqlite::open(matches.value_of("SQLITE3").unwrap())?;
    match matches.value_of("FMT").unwrap() {
        "graphflow" => {
            let (path, name) = split_path(matches.value_of("OUTPUT").unwrap())?;
            let mut vertices_buf =
                BufWriter::new(File::create(path.join(format!("{}_vertices.csv", name)))?);
            let mut edges_buf =
                BufWriter::new(File::create(path.join(format!("{}_edges.csv", name)))?);
            sqlite3_to_graphflow(&conn, &mut vertices_buf, &mut edges_buf)?;
        }
        "neo4j" => {
            let (path, name) = split_path(matches.value_of("OUTPUT").unwrap())?;
            let mut vertices_buf =
                BufWriter::new(File::create(path.join(format!("{}_vertices.csv", name)))?);
            let mut edges_buf =
                BufWriter::new(File::create(path.join(format!("{}_edges.csv", name)))?);
            sqlite3_to_neo4j(&conn, &mut vertices_buf, &mut edges_buf)?;
        }
        "sqlite3" => {
            File::create(matches.value_of("OUTPUT").unwrap())?;
            let new_conn = sqlite::open(matches.value_of("OUTPUT").unwrap())?;
            sqlite3_to_sqlite3(
                &conn,
                &new_conn,
                matches.value_of("num-vlabels").unwrap().parse()?,
                matches.value_of("num-elabels").unwrap().parse()?,
            );
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn split_path(path: &str) -> Result<(&Path, &str), InvalidPath> {
    let output = Path::new(path);
    Ok((
        output.parent().ok_or(InvalidPath)?,
        output
            .file_name()
            .ok_or(InvalidPath)?
            .to_str()
            .ok_or(InvalidPath)?,
    ))
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
        "cypher" => {
            let mut gisp = String::new();
            BufReader::new(File::open(matches.value_of("GISP").unwrap())?)
                .read_to_string(&mut gisp)?;
            writeln!(
                &mut BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?),
                "{}",
                gisp_to_cypher(&parse(&gisp)?)
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
                .arg(Arg::with_name("FMT").required(true).possible_values(&[
                    "graphflow",
                    "neo4j",
                    "sqlite3",
                ]))
                .arg(Arg::with_name("SQLITE3").required(true))
                .arg(Arg::with_name("OUTPUT").required(true))
                .arg(
                    Arg::with_name("num-vlabels")
                        .long("num-vlabels")
                        .takes_value(true)
                        .required_if("FMT", "sqlite3"),
                )
                .arg(
                    Arg::with_name("num-elabels")
                        .long("num-elabels")
                        .takes_value(true)
                        .required_if("FMT", "sqlite3"),
                ),
        )
        .subcommand(
            SubCommand::with_name("convertgisp")
                .about("Converts gisp file to other format")
                .arg(
                    Arg::with_name("FMT")
                        .required(true)
                        .possible_values(&["cypher", "graphflow"]),
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
