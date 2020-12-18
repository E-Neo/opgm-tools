use crate::types::{VId, VIdVIdELabel, VIdVLabel};
use itertools::Itertools;
use memmap::MmapMut;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    mem::size_of,
    path::Path,
};

pub fn read_edges_file<P: AsRef<Path>>(path: P) -> std::io::Result<(File, File)> {
    let input_file = File::open(path)?;
    let mut vertices = BufWriter::new(tempfile::tempfile()?);
    let mut edges = BufWriter::new(tempfile::tempfile()?);

    let re = Regex::new(r"^(\d+)\D+(\d+)\D*$").unwrap();
    let mut vertices_temp = BufWriter::new(tempfile::tempfile()?);
    for lines_item in BufReader::new(input_file).lines() {
        let lines_item = lines_item?;
        let line = lines_item.trim();
        if let Some(caps) = re.captures(line) {
            let src: VId = caps[1].parse().unwrap();
            let dst: VId = caps[2].parse().unwrap();
            vertices_temp.write_all(unsafe {
                std::slice::from_raw_parts(
                    &[src, dst] as *const _ as *const u8,
                    2 * size_of::<VId>(),
                )
            })?;
            edges.write_all(unsafe {
                std::slice::from_raw_parts(
                    &VIdVIdELabel(src, dst, 0) as *const _ as *const u8,
                    size_of::<VIdVIdELabel>(),
                )
            })?;
        }
    }

    let vertices_temp = vertices_temp.into_inner()?;
    let mut mmap = unsafe { MmapMut::map_mut(&vertices_temp)? };
    let vids = unsafe {
        std::slice::from_raw_parts_mut(mmap.as_mut_ptr() as *mut VId, mmap.len() / size_of::<VId>())
    };
    vids.sort();
    for vid in vids.iter().dedup() {
        let vid: VId = *vid;
        vertices.write_all(unsafe {
            std::slice::from_raw_parts(
                &VIdVLabel(vid, 0) as *const _ as *const u8,
                size_of::<VIdVLabel>(),
            )
        })?;
    }
    let (mut vertices, mut edges) = (vertices.into_inner()?, edges.into_inner()?);
    vertices.seek(SeekFrom::Start(0))?;
    edges.seek(SeekFrom::Start(0))?;
    Ok((vertices, edges))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::{NamedTempFile, TempPath};

    fn create_edges_file(edges: &[(VId, VId)]) -> TempPath {
        let mut file = BufWriter::new(NamedTempFile::new().unwrap());
        writeln!(&mut file, "# u1, u2").unwrap();
        for (u1, u2) in edges {
            writeln!(&mut file, "{} {}", u1, u2).unwrap();
        }
        file.into_inner().unwrap().into_temp_path()
    }

    #[test]
    fn test_read_edges_file() {
        let path = create_edges_file(&[(1, 2), (1, 3), (1, 4), (5, 6), (5, 7), (5, 8)]);
        let (vertices, edges) = read_edges_file(path).unwrap();
        let mut buffer = Vec::new();
        BufReader::new(vertices).read_to_end(&mut buffer).unwrap();
        let buffer_tmp: Vec<VIdVLabel> = (1..=8)
            .into_iter()
            .map(|vid| VIdVLabel(vid as VId, 0))
            .collect();
        assert_eq!(buffer, unsafe {
            std::slice::from_raw_parts(
                buffer_tmp.as_ptr() as *const _ as *const u8,
                buffer_tmp.len() * size_of::<VIdVLabel>(),
            )
        });
        let mut buffer = Vec::new();
        BufReader::new(edges).read_to_end(&mut buffer).unwrap();
        let buffer_tmp: Vec<VIdVIdELabel> = [(1, 2), (1, 3), (1, 4), (5, 6), (5, 7), (5, 8)]
            .iter()
            .map(|&(src, dst)| VIdVIdELabel(src as VId, dst as VId, 0))
            .collect();
        assert_eq!(buffer, unsafe {
            std::slice::from_raw_parts(
                buffer_tmp.as_ptr() as *const u8,
                buffer_tmp.len() * size_of::<VIdVIdELabel>(),
            )
        });
    }
}
