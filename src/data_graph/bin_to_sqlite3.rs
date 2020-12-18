use crate::{
    data_graph::write_sqlite3,
    types::{VIdVIdELabel, VIdVLabel},
};
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
                vertices.as_ptr() as *const VIdVLabel,
                vertices.len() / size_of::<VIdVLabel>(),
            )
        }
        .iter()
        .map(|&VIdVLabel(vid, vlabel)| (vid, vlabel)),
        unsafe {
            std::slice::from_raw_parts(
                edges.as_ptr() as *const VIdVIdELabel,
                edges.len() / size_of::<VIdVIdELabel>(),
            )
        }
        .iter()
        .map(|&VIdVIdELabel(src, dst, elabel)| (src, dst, elabel)),
    )?;
    Ok((num_vertices, num_edges))
}
