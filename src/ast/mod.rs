//! Data information to build the AST
//! And some functions to work with AST
//!

pub(crate) mod replace;

// -------------------------------------------------------------------------------------
//  T Y P E S

/// template to transform and
/// nodes to be used on transform
#[derive(Debug, PartialEq)]
pub struct Transf2 {
    /// template info to replace
    pub(crate) template: crate::parser::expression::ReplTemplate,
    /// nodes on witch will be applied the transformation
    pub(crate) nodes: Vec<Node>,
}

/// Information of a node
#[derive(Debug, PartialEq)]
pub enum Node {
    /// The node is terminal (atom) with a name
    Val(String),
    /// The node is not terminal (rule)
    /// with a name and a vec of nodes
    Rule((String, Vec<Node>)),
    /// Named nodes
    Named((String, Vec<Node>)),
    /// Named nodes
    Transf2(Transf2),
    /// Reached end of file
    EOF,
}
