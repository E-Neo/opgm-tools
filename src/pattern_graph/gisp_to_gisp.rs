use crate::{
    pattern_graph::Ast,
    types::{ELabel, VId, VLabel},
    SEED,
};
use rand::{Rng, SeedableRng};
use std::collections::HashSet;

pub fn gisp_to_gisp(ast: &Ast, num_vlabels: usize, num_elabels: usize) -> String {
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(SEED);
    Ast::new(
        ast.vertices()
            .iter()
            .map(|&(vid, _)| (vid, rng.gen_range(0..num_vlabels as VLabel)))
            .collect(),
        ast.arcs()
            .iter()
            .map(|&(src, dst, _)| (src, dst, rng.gen_range(0..num_elabels as ELabel)))
            .collect(),
        ast.edges()
            .iter()
            .map(|&(src, dst, _)| (src, dst, rng.gen_range(0..num_elabels as ELabel)))
            .collect(),
        ast.constraint().map(|expr| expr.clone()),
    )
    .to_string()
}

pub fn gisp_to_star(ast: &Ast, root: VId) -> String {
    let mut vertex_set = HashSet::with_capacity(ast.vertices().len());
    let mut arcs = Vec::with_capacity(ast.arcs().len());
    let mut edges = Vec::with_capacity(ast.edges().len());
    for &(src, dst, elabel) in ast.arcs() {
        if src == root || dst == root {
            arcs.push((src, dst, elabel));
            vertex_set.insert(src);
            vertex_set.insert(dst);
        }
    }
    for &(src, dst, elabel) in ast.edges() {
        if src == root || dst == root {
            edges.push((src, dst, elabel));
            vertex_set.insert(src);
            vertex_set.insert(dst);
        }
    }
    Ast::new(
        ast.vertices()
            .iter()
            .filter_map(|&(vid, vlabel)| {
                if vertex_set.contains(&vid) {
                    Some((vid, vlabel))
                } else {
                    None
                }
            })
            .collect(),
        arcs,
        edges,
        ast.constraint().map(|expr| expr.clone()),
    )
    .to_string()
}
