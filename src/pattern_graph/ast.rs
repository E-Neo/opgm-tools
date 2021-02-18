//! Abstract Syntax Tree

use crate::types::{ELabel, VId, VLabel};
use derive_more::Display;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub vertices: Vec<(VId, VLabel)>,
    pub arcs: Vec<(VId, VId, ELabel)>,
    pub edges: Vec<(VId, VId, ELabel)>,
    pub constraint: Option<Expr>,
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

    pub fn constraint(&self) -> Option<&Expr> {
        self.constraint.as_ref()
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(match")?;
        if !self.vertices().is_empty() {
            write!(
                f,
                " (vertices {})",
                self.vertices()
                    .iter()
                    .map(|&(vid, vlabel)| format!("(u{} {})", vid, vlabel))
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        if !self.arcs().is_empty() {
            write!(
                f,
                " (arcs {})",
                self.arcs()
                    .iter()
                    .map(|&(src, dst, elabel)| format!("(u{} u{} {})", src, dst, elabel))
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        if !self.edges().is_empty() {
            write!(
                f,
                " (edges {})",
                self.arcs()
                    .iter()
                    .map(|&(src, dst, elabel)| format!("(u{} u{} {})", src, dst, elabel))
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        if let Some(expr) = self.constraint() {
            write!(f, " (where {})", expr)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Display, Clone, PartialEq)]
pub enum Expr {}
