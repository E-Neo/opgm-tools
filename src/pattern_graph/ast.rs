//! Abstract Syntax Tree

use crate::types::{ELabel, VId, VLabel};

#[derive(Debug, PartialEq)]
pub struct Ast {
    vertices: Vec<(VId, VLabel)>,
    arcs: Vec<(VId, VId, ELabel)>,
    edges: Vec<(VId, VId, ELabel)>,
    constraint: Option<Expr>,
}

impl Ast {
    pub fn new(
        vertices: Vec<(VId, VLabel)>,
        arcs: Vec<(VId, VId, ELabel)>,
        edges: Vec<(VId, VId, ELabel)>,
        constraint: Option<Expr>,
    ) -> Self {
        Self {
            vertices,
            arcs,
            edges,
            constraint,
        }
    }

    pub fn vertices(&self) -> &[(VId, VLabel)] {
        &self.vertices
    }

    pub fn arcs(&self) -> &[(VId, VId, ELabel)] {
        &self.arcs
    }

    pub fn edges(&self) -> &[(VId, VId, ELabel)] {
        &self.edges
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {}
