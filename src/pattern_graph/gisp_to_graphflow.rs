use crate::pattern_graph::Ast;
use std::collections::HashMap;

pub fn gisp_to_graphflow(ast: &Ast) -> String {
    let vid_vlabels: HashMap<_, _> = ast
        .vertices()
        .iter()
        .map(|&(vid, vlabel)| (vid, vlabel))
        .collect();
    let results: Vec<String> = ast
        .arcs()
        .iter()
        .map(|&(src, dst, elabel)| {
            format!(
                "(u{}:{})-[{}]->(u{}:{})",
                src,
                vid_vlabels.get(&src).unwrap(),
                elabel + 1,
                dst,
                vid_vlabels.get(&dst).unwrap()
            )
        })
        .collect();
    results.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern_graph::parse;

    #[test]
    fn test_gisp_to_graphflow() {
        assert_eq!(
            gisp_to_graphflow(
                &parse("(match (vertices (u1 1) (u2 2) (u3 3)) (arcs (u1 u2 12) (u1 u3 13)))")
                    .unwrap()
            ),
            "(u1:1)-[13]->(u2:2),(u1:1)-[14]->(u3:3)"
        );
    }
}
