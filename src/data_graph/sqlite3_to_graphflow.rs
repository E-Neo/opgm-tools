use crate::types::VLabel;
use memmap::MmapMut;
use std::{io::Write, mem::size_of};

pub fn sqlite3_to_graphflow<W: Write>(
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
    if let Some(max_vid) = select_max_vid(conn) {
        let temp_vlabels = tempfile::tempfile()?;
        temp_vlabels.set_len((max_vid + 1) as u64 * size_of::<VLabel>() as u64)?;
        let mut mmap = unsafe { MmapMut::map_mut(&temp_vlabels)? };
        let vlabels = unsafe {
            std::slice::from_raw_parts_mut(
                mmap.as_mut_ptr() as *mut VLabel,
                mmap.len() / size_of::<VLabel>(),
            )
        };
        let mut stat = conn.prepare("SELECT * FROM vertices").unwrap();
        while let sqlite::State::Row = stat.next().unwrap() {
            let vid: i64 = stat.read(0).unwrap();
            let vlabel: i64 = stat.read(1).unwrap();
            vlabels[vid as usize] = vlabel as VLabel;
        }
        for (vid, &vlabel) in vlabels.iter().enumerate() {
            writeln!(buf, "{},{}", vid, vlabel)?;
        }
        Ok(max_vid as usize + 1)
    } else {
        Ok(0)
    }
}

fn select_max_vid(conn: &sqlite::Connection) -> Option<i64> {
    let mut stat = conn.prepare("SELECT MAX(vid) FROM vertices").unwrap();
    if let sqlite::State::Row = stat.next().unwrap() {
        let vid: i64 = stat.read(0).unwrap();
        Some(vid)
    } else {
        None
    }
}

fn write_edges<W: Write>(conn: &sqlite::Connection, buf: &mut W) -> std::io::Result<usize> {
    let mut count = 0;
    let mut stat = conn.prepare("SELECT * FROM edges").unwrap();
    while let sqlite::State::Row = stat.next().unwrap() {
        let src: i64 = stat.read(0).unwrap();
        let dst: i64 = stat.read(1).unwrap();
        let elabel: i64 = stat.read(2).unwrap();
        writeln!(buf, "{},{},{}", src, dst, elabel + 1)?;
        count += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_graph::write_sqlite3;

    #[test]
    fn test_sqlite3_to_graphflow() {
        let conn = sqlite::open(":memory:").unwrap();
        let vertices = vec![(1, 1), (2, 2), (3, 3)];
        let edges = vec![(1, 2, 12), (1, 3, 13)];
        write_sqlite3(&conn, vertices.clone(), edges.clone()).unwrap();
        let (mut vertices_buf, mut edges_buf) = (Vec::new(), Vec::new());
        sqlite3_to_graphflow(&conn, &mut vertices_buf, &mut edges_buf).unwrap();
        let (mut vertices_temp, mut edges_temp) = (Vec::new(), Vec::new());
        writeln!(&mut vertices_temp, "{},{}", 0, 0).unwrap();
        for (vid, vlabel) in vertices {
            writeln!(&mut vertices_temp, "{},{}", vid, vlabel).unwrap();
        }
        for (src, dst, elabel) in edges {
            writeln!(&mut edges_temp, "{},{},{}", src, dst, elabel + 1).unwrap();
        }
        assert_eq!(vertices_buf, vertices_temp);
        assert_eq!(edges_buf, edges_temp);
    }
}
