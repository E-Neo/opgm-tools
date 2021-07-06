use crate::types::{VId, VLabel};
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn snap_edges_to_sqlite3(
    conn: &sqlite::Connection,
    edges_file: &File,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    create_tables(conn)?;
    let mut vids = BTreeSet::new();
    let mut num_edges = 0;
    conn.execute("BEGIN")?;
    for lines_item in BufReader::new(edges_file).lines() {
        let line = lines_item?;
        if let &[Ok(src), Ok(dst)] = line
            .split_whitespace()
            .map(|n| n.parse::<VId>())
            .collect::<Vec<_>>()
            .as_slice()
        {
            vids.insert(src);
            vids.insert(dst);
            conn.execute(format!("INSERT INTO edges VALUES ({}, {}, 0)", src, dst))?;
            num_edges += 1;
        }
    }
    conn.execute("END")?;
    let num_vertices = insert_vertices(conn, vids.into_iter().map(|vid| (vid, 0)))?;
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
