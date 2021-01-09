use crate::pattern_graph::Ast;
use std::collections::HashMap;

pub fn gisp_to_cypher(ast: &Ast) -> String {
    let vid_vlabels: HashMap<_, _> = ast
        .vertices()
        .iter()
        .map(|&(vid, vlabel)| (vid, vlabel))
        .collect();
    let vertices: Vec<String> = ast
        .vertices()
        .iter()
        .map(|&(vid, _)| format!("ID(u{})", vid))
        .collect();
    let arcs: Vec<String> = ast
        .arcs()
        .iter()
        .map(|&(src, dst, elabel)| {
            format!(
                "(u{}:`{}`)-[:`{}`]->(u{}:`{}`)",
                src,
                vid_vlabels.get(&src).unwrap(),
                elabel,
                dst,
                vid_vlabels.get(&dst).unwrap()
            )
        })
        .collect();
    format!("MATCH {} RETURN {}", arcs.join(", "), vertices.join(", "))
}
