//! # RSX Parser
//!
//! Parses RSX syntax into an AST that can be emitted as Rust code.

use syn::braced;
use syn::parse::discouraged::Speculative;
use syn::{
    Expr, Ident, LitStr, Pat, Path, Result, Token,
    parse::{Parse, ParseStream},
    token::Brace,
};

/// Root of an RSX expression
pub struct RsxRoot {
    /// Top-level nodes in the RSX
    pub nodes: Vec<RsxNode>,
}

impl Parse for RsxRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            nodes.push(input.parse()?);
        }
        Ok(Self { nodes })
    }
}

/// A single component prop: `name: value`
pub struct RsxProp {
    /// Property name
    pub name: Ident,
    /// Property value expression
    pub value: Expr,
}

/// AST node representing different RSX constructs
pub enum RsxNode {
    /// String literal: `"text"`
    Text(LitStr),
    /// Expression node: `{expr}`
    Expr(Expr),
    /// Component instantiation: `Component { prop: value, ... }`
    Component {
        /// Component path (e.g., `my_module::MyComponent`)
        path: Path,
        /// Component properties
        props: Vec<RsxProp>,
        /// Child nodes
        children: Vec<RsxNode>,
    },
    /// Mount lifecycle: `@mount`
    Mount(Ident),
    /// Conditional rendering: `@if condition { ... }`
    If {
        /// Dependencies to track for reactivity
        deps: Vec<Dep>,
        /// Condition expression
        cond: Expr,
        /// Child nodes to render if true
        children: Vec<RsxNode>,
    },
    /// Loop rendering: `@for pattern in expr { ... }`
    For {
        /// Dependencies to track for reactivity
        deps: Vec<Dep>,
        /// Loop pattern (e.g., `(key, value)`)
        pat: Pat,
        /// Iterable expression
        expr: Expr,
        /// Child nodes to render for each iteration
        children: Vec<RsxNode>,
    },
}

pub struct Dep {
    pub ident: Ident,
    pub pat: Option<Pat>,
}

fn parse_deps(input: ParseStream) -> Result<Vec<Dep>> {
    let mut deps = Vec::new();

    if input.peek(Token![%]) {
        input.parse::<Token![%]>()?;
        loop {
            let ident: Ident = input.parse()?;
            let pat = if input.peek(Token![as]) {
                input.parse::<Token![as]>()?;
                // Patterns in syn 2.0 are parsed via `Pat::parse_multi`
                // instead of implementing the `Parse` trait directly.
                Some(Pat::parse_multi(input)?)
            } else {
                None
            };

            deps.push(Dep { ident, pat });

            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
        }
    }

    Ok(deps)
}

impl Parse for RsxNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // @{ $expr }
        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;

            let content;
            braced!(content in input);

            let expr: Expr = content.parse()?;
            return Ok(RsxNode::Expr(expr));
        }

        // !$ident
        if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            let mount: Ident = input.parse()?;
            return Ok(RsxNode::Mount(mount));
        }

        let deps = parse_deps(input)?;

        // %$dep if $expr { $rsx }
        if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            let cond: Expr = input.parse()?;
            let content;
            braced!(content in input);
            let children = parse_children(&content)?;
            return Ok(RsxNode::If {
                deps,
                cond,
                children,
            });
        }

        // %$dep for $pat in $expr { $rsx }
        if input.peek(Token![for]) {
            input.parse::<Token![for]>()?;
            let pat: Pat = Pat::parse_multi(input)?;
            input.parse::<Token![in]>()?;
            let expr: Expr = input.parse()?;
            let content;
            braced!(content in input);
            let children = parse_children(&content)?;
            return Ok(RsxNode::For {
                deps,
                pat,
                expr,
                children,
            });
        }

        // "%$dep for $pat in $expr { $rsx }"
        if input.peek(LitStr) {
            return Ok(RsxNode::Text(input.parse()?));
        }

        parse_component_invocation(input)
    }
}

/// Parses Path or Path { ... } into RsxNode::Component.
fn parse_component_invocation(input: ParseStream) -> Result<RsxNode> {
    let path: Path = input.parse()?;

    if !input.peek(Brace) {
        return Ok(RsxNode::Component {
            path,
            props: Vec::new(),
            children: Vec::new(),
        });
    }

    let content;
    braced!(content in input);

    let (props, children) = parse_props_and_children(&content)?;

    Ok(RsxNode::Component {
        path,
        props,
        children,
    })
}

fn parse_props_and_children(input: ParseStream) -> Result<(Vec<RsxProp>, Vec<RsxNode>)> {
    Ok((parse_props(input)?, RsxRoot::parse(input)?.nodes))
}

fn parse_props(input: ParseStream) -> Result<Vec<RsxProp>> {
    let mut props = Vec::new();

    while !input.is_empty() {
        let fork = input.fork();

        let path: Path = match fork.parse() {
            Ok(p) => p,
            Err(_) => break,
        };

        if fork.peek(Token![:]) {
            let name = path
                .get_ident()
                .cloned()
                .ok_or_else(|| fork.error("prop name must be a single identifier"))?;

            fork.parse::<Token![:]>()?;
            let value: Expr = fork.parse()?;

            input.advance_to(&fork);

            props.push(RsxProp { name, value });

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        } else {
            // ❌ do not commit → nothing consumed
            break;
        }
    }

    Ok(props)
}

fn parse_children(input: ParseStream) -> Result<Vec<RsxNode>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        nodes.push(input.parse()?);
    }
    Ok(nodes)
}
