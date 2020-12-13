use crate::data_graph::write_sqlite3;
use memmap::Mmap;
use std::{fs::File, mem::size_of};

pub fn bin_to_sqlite3(
    conn: &sqlite::Connection,
    vertices: &File,
    edges: &File,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let vertices = unsafe { Mmap::map(vertices)? };
    let edges = unsafe { Mmap::map(edges)? };
    let (num_vertices, num_edges) = write_sqlite3(
        conn,
        unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr() as *const i64,
                vertices.len() / size_of::<i64>(),
            )
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[1]))
        },
        unsafe {
            std::slice::from_raw_parts(edges.as_ptr() as *const i64, edges.len() / size_of::<i64>())
                .chunks_exact(3)
                .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        },
    )?;
    Ok((num_vertices, num_edges))
}
