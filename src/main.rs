use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};
use derive_more::{Display, Error};
use opgm_tools::{
    data_graph::{
        snap_edges_to_sqlite3, sqlite3_to_graphflow, sqlite3_to_neo4j, sqlite3_to_sqlite3,
    },
    pattern_graph::{gisp_to_cypher, gisp_to_gisp, gisp_to_graphflow, gisp_to_star, parse},
    types::VId,
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
            snap_edges_to_sqlite3(
                &sqlite::open(matches.value_of("SQLITE3").unwrap())?,
                &File::open(matches.value_of("INPUT").unwrap())?,
            )?;
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
    let mut gisp = String::new();
    BufReader::new(File::open(matches.value_of("GISP").unwrap())?).read_to_string(&mut gisp)?;
    let ast = parse(&gisp)?;
    let mut output = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?);
    match matches.value_of("FMT").unwrap() {
        "gisp" => writeln!(
            &mut output,
            "{}",
            gisp_to_gisp(
                &ast,
                matches.value_of("num-vlabels").unwrap().parse()?,
                matches.value_of("num-elabels").unwrap().parse()?
            )
        )?,
        "graphflow" => writeln!(&mut output, "{}", gisp_to_graphflow(&ast))?,
        "cypher" => writeln!(&mut output, "{}", gisp_to_cypher(&ast))?,
        _ => unreachable!(),
    }
    Ok(())
}

fn handle_gispinfo(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let gisp_path = Path::new(matches.value_of("GISP").unwrap());
    let mut gisp = String::new();
    BufReader::new(File::open(gisp_path)?).read_to_string(&mut gisp)?;
    let ast = parse(&gisp)?;
    println!("num_vertices: {}", ast.vertices().len());
    println!("num_arcs: {}", ast.arcs().len());
    println!("num_edges: {}", ast.edges().len());
    Ok(())
}

fn handle_stars(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let gisp_path = Path::new(matches.value_of("GISP").unwrap());
    let mut gisp = String::new();
    BufReader::new(File::open(gisp_path)?).read_to_string(&mut gisp)?;
    let mut ast = parse(&gisp)?;
    let outdir = Path::new(matches.value_of("OUTDIR").unwrap());
    let roots: Vec<VId> = matches
        .value_of("ROOTS")
        .unwrap()
        .split(",")
        .map(|v| v.parse::<VId>().unwrap())
        .collect();
    for root in roots {
        writeln!(
            &mut BufWriter::new(File::create(outdir.join(&format!(
                "{}_{}.{}",
                gisp_path.file_stem().unwrap().to_string_lossy(),
                root,
                gisp_path.extension().unwrap().to_string_lossy()
            )))?),
            "{}",
            gisp_to_star(&ast, root)
        )?;
        match matches.value_of("method").unwrap() {
            "opgm" => {}
            "stwig" => {
                ast.arcs = ast
                    .arcs
                    .into_iter()
                    .filter(|&(src, dst, _)| src != root && dst != root)
                    .collect();
                ast.edges = ast
                    .edges
                    .into_iter()
                    .filter(|&(src, dst, _)| src != root && dst != root)
                    .collect();
            }
            _ => unreachable!(),
        }
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
                .arg(Arg::with_name("FMT").required(true).possible_values(&[
                    "cypher",
                    "gisp",
                    "graphflow",
                ]))
                .arg(Arg::with_name("GISP").required(true))
                .arg(Arg::with_name("OUTPUT").required(true))
                .arg(
                    Arg::with_name("num-vlabels")
                        .long("num-vlabels")
                        .takes_value(true)
                        .required_if("FMT", "gisp"),
                )
                .arg(
                    Arg::with_name("num-elabels")
                        .long("num-elabels")
                        .takes_value(true)
                        .required_if("FMT", "gisp"),
                ),
        )
        .subcommand(SubCommand::with_name("gispinfo").arg(Arg::with_name("GISP").required(true)))
        .subcommand(
            SubCommand::with_name("stars")
                .arg(Arg::with_name("GISP").required(true))
                .arg(Arg::with_name("OUTDIR").required(true))
                .arg(Arg::with_name("ROOTS").required(true))
                .arg(
                    Arg::with_name("method")
                        .long("method")
                        .default_value("opgm")
                        .possible_values(&["opgm", "stwig"]),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("createdb") {
        handle_createdb(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("convertdb") {
        handle_convertdb(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("convertgisp") {
        handle_convertgisp(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("gispinfo") {
        handle_gispinfo(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("stars") {
        handle_stars(matches)?;
    }
    Ok(())
}
