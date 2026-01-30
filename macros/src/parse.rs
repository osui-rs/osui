use syn::braced;
use syn::{
    Expr, Ident, LitStr, Pat, Path, Result, Token,
    parse::{Parse, ParseStream},
    token::Brace,
};

pub struct RsxRoot {
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

/// A single prop: `ident: expr`.
pub struct RsxProp {
    pub name: Ident,
    pub value: Expr,
}

pub enum RsxNode {
    Text(LitStr),
    Expr(Expr),
    Component {
        path: Path,
        props: Vec<RsxProp>,
        children: Vec<RsxNode>,
    },
    Mount(Ident),
    If {
        deps: Vec<Dep>,
        cond: Expr,
        children: Vec<RsxNode>,
    },
    For {
        deps: Vec<Dep>,
        pat: Pat,
        expr: Expr,
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
        // --- @{ expr } ---
        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;

            let content;
            braced!(content in input);

            let expr: Expr = content.parse()?;
            return Ok(RsxNode::Expr(expr));
        }

        // --- existing code ---
        if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            let mount: Ident = input.parse()?;
            return Ok(RsxNode::Mount(mount));
        }

        let deps = parse_deps(input)?;

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

/// Parses comma-separated "ident: expr" (props) and RsxNode (children) inside braces.
fn parse_props_and_children(input: ParseStream) -> Result<(Vec<RsxProp>, Vec<RsxNode>)> {
    let mut props = Vec::new();
    let mut children = Vec::new();

    while !input.is_empty() {
        if input.peek(LitStr) {
            children.push(input.parse()?);
        } else {
            let path: Path = input.parse()?;

            if input.peek(Token![:]) {
                let name = path
                    .get_ident()
                    .cloned()
                    .ok_or_else(|| input.error("prop name must be a single identifier"))?;
                input.parse::<Token![:]>()?;
                let value: Expr = input.parse()?;
                props.push(RsxProp { name, value });
            } else if input.peek(Brace) {
                let nested_content;
                braced!(nested_content in input);
                let (nested_props, nested_children) = parse_props_and_children(&nested_content)?;
                children.push(RsxNode::Component {
                    path,
                    props: nested_props,
                    children: nested_children,
                });
            } else {
                children.push(RsxNode::Component {
                    path,
                    props: Vec::new(),
                    children: Vec::new(),
                });
            }
        }

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }
    }

    Ok((props, children))
}

fn parse_children(input: ParseStream) -> Result<Vec<RsxNode>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        nodes.push(input.parse()?);
    }
    Ok(nodes)
}
