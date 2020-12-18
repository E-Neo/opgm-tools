use crate::{
    pattern_graph::Ast,
    types::{ELabel, VId, VLabel},
};
use pest::Parser as PestParser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pattern_graph/gisp.pest"]
struct Parser;

pub fn parse(source: &str) -> Result<Ast, pest::error::Error<Rule>> {
    Ok(Parser::parse(Rule::Query, source)?
        .next()
        .map(|query| query_to_ast(query))
        .unwrap())
}

fn query_to_ast(pair: pest::iterators::Pair<Rule>) -> Ast {
    let (mut vertices, mut arcs, mut edges, constraint) = (vec![], vec![], vec![], None);
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Vertices => {
                for vertex in pair.into_inner() {
                    let mut pair = vertex.into_inner();
                    let vid: VId = pair.next().unwrap().as_str()[1..].parse().unwrap();
                    let vlabel: VLabel = pair.next().unwrap().as_str().parse().unwrap();
                    vertices.push((vid, vlabel))
                }
            }
            Rule::Arcs => {
                for arc in pair.into_inner() {
                    let mut pair = arc.into_inner();
                    let src: VId = pair.next().unwrap().as_str()[1..].parse().unwrap();
                    let dst: VId = pair.next().unwrap().as_str()[1..].parse().unwrap();
                    let elabel: ELabel = pair.next().unwrap().as_str().parse().unwrap();
                    arcs.push((src, dst, elabel))
                }
            }
            Rule::Edges => {
                for edge in pair.into_inner() {
                    let mut pair = edge.into_inner();
                    let src: VId = pair.next().unwrap().as_str()[1..].parse().unwrap();
                    let dst: VId = pair.next().unwrap().as_str()[1..].parse().unwrap();
                    let elabel: ELabel = pair.next().unwrap().as_str().parse().unwrap();
                    edges.push((src, dst, elabel))
                }
            }
            Rule::Where => todo!(),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ast::new(vertices, arcs, edges, constraint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        assert_eq!(
            parse(
                "\
(match (vertices (u1 1) (u2 2) (u3 3))
       (arcs (u1 u2 12) (u1 u3 13))
       (edges (u2 u3 23)))"
            ),
            Ok(Ast::new(
                vec![(1, 1), (2, 2), (3, 3)],
                vec![(1, 2, 12), (1, 3, 13)],
                vec![(2, 3, 23)],
                None
            ))
        );
    }
}
