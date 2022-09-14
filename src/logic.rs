use crate::expression::*;
use crate::node::*;
use crate::storage::*;

/*
 * this function supposes that some expression was parsed from text,
 * when this occurs the context is populated with the root_expr and 
 * ALL sub-expressions of root_expr.
*/

/// context <- expression
/// registry <- node


pub fn grow_tree(root_expr: Expression, context: &Context) -> (Node, Registry) {
    let mut root = Node::new(0, root_expr.id(), None);

    let mut registry: Registry = Storage::new();
    // let root_id = registry.add(root);

    (root, registry)
}
