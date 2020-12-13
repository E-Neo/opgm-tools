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
            let src: i64 = caps[1].parse().unwrap();
            let dst: i64 = caps[2].parse().unwrap();
            vertices_temp.write_all(unsafe {
                std::slice::from_raw_parts(
                    &[src, dst] as *const _ as *const u8,
                    2 * size_of::<i64>(),
                )
            })?;
            edges.write_all(unsafe {
                std::slice::from_raw_parts(
                    &[src, dst, 0] as *const _ as *const u8,
                    3 * size_of::<i64>(),
                )
            })?;
        }
    }

    let vertices_temp = vertices_temp.into_inner()?;
    let mut mmap = unsafe { MmapMut::map_mut(&vertices_temp)? };
    let vids = unsafe {
        std::slice::from_raw_parts_mut(mmap.as_mut_ptr() as *mut i64, mmap.len() / size_of::<i64>())
    };
    vids.sort();
    for vid in vids.iter().dedup() {
        let vid: i64 = *vid;
        vertices.write_all(unsafe {
            std::slice::from_raw_parts(&[vid, 0] as *const _ as *const u8, 2 * size_of::<i64>())
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

    fn create_edges_file(edges: &[(i64, i64)]) -> TempPath {
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
        assert_eq!(
            unsafe {
                std::slice::from_raw_parts(
                    buffer.as_ptr() as *const i64,
                    buffer.len() / size_of::<i64>(),
                )
            },
            &[1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0]
        );
        let mut buffer = Vec::new();
        BufReader::new(edges).read_to_end(&mut buffer).unwrap();
        assert_eq!(
            unsafe {
                std::slice::from_raw_parts(
                    buffer.as_ptr() as *const i64,
                    buffer.len() / size_of::<i64>(),
                )
            },
            &[1, 2, 0, 1, 3, 0, 1, 4, 0, 5, 6, 0, 5, 7, 0, 5, 8, 0]
        );
    }
}
