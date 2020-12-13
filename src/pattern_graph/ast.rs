//! Abstract Syntax Tree

#[derive(Debug, PartialEq)]
pub struct Ast {
    vertices: Vec<(i64, i64)>,
    arcs: Vec<(i64, i64, i64)>,
    edges: Vec<(i64, i64, i64)>,
    constraint: Option<Expr>,
}

impl Ast {
    pub fn new(
        vertices: Vec<(i64, i64)>,
        arcs: Vec<(i64, i64, i64)>,
        edges: Vec<(i64, i64, i64)>,
        constraint: Option<Expr>,
    ) -> Self {
        Self {
            vertices,
            arcs,
            edges,
            constraint,
        }
    }

    pub fn vertices(&self) -> &[(i64, i64)] {
        &self.vertices
    }

    pub fn arcs(&self) -> &[(i64, i64, i64)] {
        &self.arcs
    }

    pub fn edges(&self) -> &[(i64, i64, i64)] {
        &self.edges
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {}
