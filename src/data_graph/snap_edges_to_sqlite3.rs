use crate::types::{VId, VLabel};
use itertools::Itertools;
use lazy_static::lazy_static;
use memmap::MmapMut;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    mem::size_of,
};

pub fn snap_edges_to_sqlite3(
    conn: &sqlite::Connection,
    edges_file: &File,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)\D+(\d+)\D*$").unwrap();
    }
    create_tables(conn)?;
    let mut vertices = BufWriter::new(tempfile::tempfile()?);
    let mut num_edges = 0;
    conn.execute("BEGIN")?;
    for lines_item in BufReader::new(edges_file).lines() {
        let lines_item = lines_item?;
        let line = lines_item.trim();
        if let Some(caps) = RE.captures(line) {
            let src: VId = caps[1].parse().unwrap();
            let dst: VId = caps[2].parse().unwrap();
            vertices.write_all(unsafe {
                std::slice::from_raw_parts(
                    &[src, dst] as *const _ as *const u8,
                    2 * size_of::<VId>(),
                )
            })?;
            conn.execute(format!("INSERT INTO edges VALUES ({}, {}, 0)", src, dst))?;
            num_edges += 1;
        }
    }
    conn.execute("END")?;
    let mut mmap = unsafe { MmapMut::map_mut(&vertices.into_inner()?)? };
    let vids = unsafe {
        std::slice::from_raw_parts_mut(mmap.as_mut_ptr() as *mut VId, mmap.len() / size_of::<VId>())
    };
    vids.sort();
    let num_vertices = insert_vertices(conn, vids.iter().dedup().map(|&vid| (vid, 0)))?;
    Ok((num_vertices, num_edges))
}

fn create_tables(conn: &sqlite::Connection) -> sqlite::Result<()> {
    conn.execute("CREATE TABLE vertices (vid INT, vlabel INT)")?;
    conn.execute("CREATE TABLE edges (src INT, dst INT, elabel INT)")
}

fn insert_vertices<VS>(conn: &sqlite::Connection, vertices: VS) -> sqlite::Result<usize>
where
    VS: IntoIterator<Item = (VId, VLabel)>,
{
    let mut num_vertices = 0;
    conn.execute("BEGIN")?;
    for (vid, vlabel) in vertices {
        conn.execute(format!("INSERT INTO vertices VALUES ({}, {})", vid, vlabel))?;
        num_vertices += 1;
    }
    conn.execute("END")?;
    Ok(num_vertices)
}
