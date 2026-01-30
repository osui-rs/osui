use syn::braced;
use syn::{
    Expr, Ident, LitStr, Pat, Result, Token,
    parse::{Parse, ParseStream},
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

pub enum RsxNode {
    Text(LitStr),
    Component(Expr),
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
            // Parse a possibly multi-pattern (e.g. with `|`) for the `for` binding.
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

        // Component: path (e.g. HelloWorld) or struct init (e.g. HelloWorld { t: 10 })
        Ok(RsxNode::Component(input.parse()?))
    }
}

fn parse_children(input: ParseStream) -> Result<Vec<RsxNode>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        nodes.push(input.parse()?);
    }
    Ok(nodes)
}
