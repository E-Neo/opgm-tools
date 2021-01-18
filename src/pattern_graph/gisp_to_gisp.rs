use crate::{
    pattern_graph::Ast,
    types::{ELabel, VLabel},
    SEED,
};
use rand::{Rng, SeedableRng};

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
