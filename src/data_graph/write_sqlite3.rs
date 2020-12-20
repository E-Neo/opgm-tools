use crate::types::{ELabel, VId, VLabel};

pub fn write_sqlite3<VS, ES>(
    conn: &sqlite::Connection,
    vertices: VS,
    edges: ES,
) -> sqlite::Result<(usize, usize)>
where
    VS: IntoIterator<Item = (VId, VLabel)>,
    ES: IntoIterator<Item = (VId, VId, ELabel)>,
{
    create_tables(conn)?;
    let num_vertices = insert_vertices(conn, vertices)?;
    let num_edges = insert_edges(conn, edges)?;
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

fn insert_edges<ES>(conn: &sqlite::Connection, edges: ES) -> sqlite::Result<usize>
where
    ES: IntoIterator<Item = (VId, VId, ELabel)>,
{
    let mut num_edges = 0;
    conn.execute("BEGIN")?;
    for (src, dst, elabel) in edges {
        conn.execute(format!(
            "INSERT INTO edges VALUES ({}, {}, {})",
            src, dst, elabel
        ))?;
        num_edges += 1;
    }
    conn.execute("END")?;
    Ok(num_edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_vertices_edges(
        conn: &sqlite::Connection,
    ) -> (Vec<(VId, VLabel)>, Vec<(VId, VId, ELabel)>) {
        let (mut vertices, mut edges) = (vec![], vec![]);
        let mut select_vertices = conn.prepare("SELECT * FROM vertices").unwrap();
        while let sqlite::State::Row = select_vertices.next().unwrap() {
            let vid: i64 = select_vertices.read(0).unwrap();
            let vlabel: i64 = select_vertices.read(1).unwrap();
            vertices.push((vid as VId, vlabel as VLabel));
        }
        let mut select_edges = conn.prepare("SELECT * FROM edges").unwrap();
        while let sqlite::State::Row = select_edges.next().unwrap() {
            let src: i64 = select_edges.read(0).unwrap();
            let dst: i64 = select_edges.read(1).unwrap();
            let elabel: i64 = select_edges.read(2).unwrap();
            edges.push((src as VId, dst as VId, elabel as ELabel));
        }
        (vertices, edges)
    }

    #[test]
    fn test_write_sqlite3() {
        let (vertices, edges) = (
            vec![
                (1, 0),
                (2, 1),
                (3, 1),
                (4, 2),
                (5, 0),
                (6, 1),
                (7, 1),
                (8, 2),
            ],
            vec![
                (1, 2, 12),
                (1, 3, 13),
                (1, 4, 14),
                (5, 6, 56),
                (5, 7, 57),
                (5, 8, 58),
            ],
        );
        let conn = sqlite::open(":memory:").unwrap();
        assert_eq!(
            write_sqlite3(
                &conn,
                vertices.iter().map(|&(vid, vlabel)| (vid, vlabel)),
                edges.iter().map(|&(u1, u2, elabel)| (u1, u2, elabel))
            )
            .unwrap(),
            (8, 6)
        );
        assert_eq!(read_vertices_edges(&conn), (vertices, edges));
    }
}
