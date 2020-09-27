use crate::ast::Node;
use idata::IString;

#[derive(Debug)]
pub(crate) struct ReplacedNodes {
    by_pos: Vec<Replaced>,
    by_name: im::HashMap<String, Replaced>,
}

impl ReplacedNodes {
    fn new() -> Self {
        ReplacedNodes {
            by_pos: Vec::<Replaced>::new(),
            by_name: im::HashMap::<String, Replaced>::new(),
        }
    }

    fn process_node(mut self, node: &Node) -> Result<Self, String> {
        let node_replaced = replace(node, None)?;
        match node {
            Node::Named((name, _nodes)) => {
                self.by_pos.push(node_replaced.clone());
                self.by_name.insert(name.clone(), node_replaced);
            }
            Node::Rule((name, _nodes)) => {
                self.by_pos.push(node_replaced.clone());
                self.by_name.insert(name.clone(), node_replaced);
            }
            Node::Val(_) | Node::Transf2(_) | Node::EOF => (),
        }
        Ok(self)
    }
}

pub(crate) fn replace(
    ast: &Node,
    fcallback: Option<&crate::FnCallBack>,
) -> Result<Replaced, String> {
    Ok(rec_replace(ast, fcallback, Replaced("".to_string()))?)
}

/// Replaced result
#[derive(Debug, Clone)]
pub struct Replaced(String);

impl Replaced {
    fn iappend(self, txt: &str) -> Self {
        Self(self.0.iappend(txt))
    }
    pub fn str(&self) -> String {
        self.0.to_string()
    }
}

fn rec_replace(
    ast: &Node,
    fcallback: Option<&crate::FnCallBack>,
    repl: Replaced,
) -> Result<Replaced, String> {
    match ast {
        Node::EOF => Ok(repl),
        Node::Val(s) => Ok(repl.iappend(s)),
        Node::Named((_, nodes)) => rec_replace_nodes(nodes, fcallback, repl),
        Node::Transf2(crate::ast::Transf2 { template, nodes }) => {
            rec_transf2_nodes(nodes, fcallback, template, repl)
        }
        Node::Rule((_, nodes)) => rec_replace_nodes(nodes, fcallback, repl),
    }
}

fn rec_replace_nodes(
    nodes: &[Node],
    fcallback: Option<&crate::FnCallBack>,
    repl: Replaced,
) -> Result<Replaced, String> {
    nodes.iter().fold(Ok(repl), |acc, node| match acc {
        Ok(repl) => rec_replace(node, fcallback, repl),
        Err(e) => Err(e),
    })
}

fn rec_transf2_nodes(
    nodes: &[Node],
    fcallback: Option<&crate::FnCallBack>,
    template: &crate::parser::expression::ReplTemplate,
    repl: Replaced,
) -> Result<Replaced, String> {
    if nodes.len() > 0 {
        let replaced_nodes = nodes.iter().fold(Ok(ReplacedNodes::new()), |acc, node| {
            acc?.process_node(node)
        })?;
        Ok(apply_transf2(fcallback, template, &replaced_nodes, repl))
    } else {
        Ok(repl)
    }
}

fn apply_transf2(
    fcallback: Option<&crate::FnCallBack>,
    template: &crate::parser::expression::ReplTemplate,
    replaced_nodes: &ReplacedNodes,
    replaced: Replaced,
) -> Replaced {
    use crate::parser::expression::ReplItem;

    template
        .0
        .iter()
        .fold(replaced, |acc, repl_item| match repl_item {
            ReplItem::Text(txt) => acc.iappend(txt),
            ReplItem::ByPos(p) => match replaced_nodes.by_pos.get(*p) {
                Some(rn) => acc.iappend(&format!("pos<{}/{}>", p, rn.0)),
                None => acc.iappend(&format!("pos<{}/missing>", p)),
            },
            ReplItem::ByName(n) => match replaced_nodes.by_name.get(n) {
                Some(rn) => acc.iappend(&format!("{}", rn.0)),
                None => acc.iappend(&format!("name<{}/missing>", n)),
            },
            ReplItem::ByNameOpt(n) => match replaced_nodes.by_name.get(n) {
                Some(rn) => acc.iappend(&format!("{}", rn.0)),
                None => acc,
            },
            ReplItem::Function(f) => acc.iappend(&replace_fn(&f, fcallback)),
        })
}

fn replace_fn(fn_name: &str, fcallback: Option<&crate::FnCallBack>) -> String {
    if let Some(fc) = fcallback {
        match fc.0(fn_name) {
            Some(replaced) => replaced,
            None => replace_internal_fn(fn_name),
        }
    } else {
        replace_internal_fn(fn_name)
    }
}

fn replace_internal_fn(fn_name: &str) -> String {
    match fn_name {
        "none" => "".to_string(),
        "endl" => "\n".to_string(),
        "spc" => " ".to_string(),
        "_" => " ".to_string(),
        "tab" => "\t".to_string(),
        "(" => "(".to_string(),
        // "now" => " ",
        _ => format!("?unknown_fn?<{}>", fn_name),
    }
}
