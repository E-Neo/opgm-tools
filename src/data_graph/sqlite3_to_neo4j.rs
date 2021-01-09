use std::io::Write;

pub fn sqlite3_to_neo4j<W: Write>(
    conn: &sqlite::Connection,
    vertices_buf: &mut W,
    edges_buf: &mut W,
) -> std::io::Result<(usize, usize)> {
    Ok((
        write_vertices(conn, vertices_buf)?,
        write_edges(conn, edges_buf)?,
    ))
}

fn write_vertices<W: Write>(conn: &sqlite::Connection, buf: &mut W) -> std::io::Result<usize> {
    let mut count = 0;
    writeln!(buf, ":ID,:LABEL")?;
    let mut stat = conn.prepare("SELECT * FROM vertices").unwrap();
    while let sqlite::State::Row = stat.next().unwrap() {
        let vid: i64 = stat.read(0).unwrap();
        let vlabel: i64 = stat.read(1).unwrap();
        writeln!(buf, "{},{}", vid, vlabel)?;
        count += 1;
    }
    Ok(count)
}

fn write_edges<W: Write>(conn: &sqlite::Connection, buf: &mut W) -> std::io::Result<usize> {
    let mut count = 0;
    writeln!(buf, ":START_ID,:END_ID,:TYPE")?;
    let mut stat = conn.prepare("SELECT * FROM edges").unwrap();
    while let sqlite::State::Row = stat.next().unwrap() {
        let src: i64 = stat.read(0).unwrap();
        let dst: i64 = stat.read(1).unwrap();
        let elabel: i64 = stat.read(2).unwrap();
        writeln!(buf, "{},{},{}", src, dst, elabel)?;
        count += 1;
    }
    Ok(count)
}
