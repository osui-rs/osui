use crate::parse::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn emit_rsx(root: RsxRoot) -> TokenStream {
    let nodes = root.nodes.iter().map(emit_node_scope);

    quote! {{
        let mut r = osui::frontend::Rsx::new();
        #(#nodes)*
        r
    }}
}

fn emit_deps(deps: &[Dep]) -> TokenStream {
    deps.iter()
        .map(|d| {
            let ident = &d.ident;
            if let Some(pat) = &d.pat {
                quote!( let #pat = #ident.clone(); )
            } else {
                quote!( let #ident = #ident.clone(); )
            }
        })
        .collect()
}

fn emit_deps_vec(deps: &[Dep]) -> TokenStream {
    let deps = deps.iter().map(|d| {
        let ident = &d.ident;

        if let Some(pat) = &d.pat {
            quote! {{
                let #pat = #ident.clone();
                Box::new(#pat) as Box<dyn HookDependency>
            }}
        } else {
            quote! {
                Box::new(#ident.clone()) as Box<dyn HookDependency>
            }
        }
    });

    quote! {
        vec![ #(#deps),* ]
    }
}

fn emit_node_scope(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Text(_) => {
            let emit = emit_node(node);
            quote! {
                r.static_scope(|scope| {#emit});
            }
        }

        RsxNode::Component(_) => {
            let emit = emit_node(node);
            quote! {
                r.static_scope(|scope| {#emit});
            }
        }

        RsxNode::Mount(m) => quote! {
            #m.mount();
        },

        RsxNode::If {
            deps,
            cond,
            children,
        } => {
            let deps_emit = emit_deps(deps);
            let deps_vec_emit = emit_deps_vec(deps);
            let kids = children.iter().map(emit_node);

            quote! {
                {
                    #deps_emit
                    r.dynamic_scope(move |scope| {
                        if #cond {
                            if scope.children.lock().unwrap().is_empty() {
                                #(#kids)*
                            }
                        } else {
                            scope.children.lock().unwrap().clear();
                        }
                    }, #deps_vec_emit);
                }
            }
        }

        RsxNode::For {
            deps,
            pat,
            expr,
            children,
        } => {
            let deps_emit = emit_deps(deps);
            let deps_vec_emit = emit_deps_vec(deps);
            let kids = children.iter().map(emit_node);

            quote! {
                {
                    #deps_emit
                    #[allow(unused_parens)]
                    r.dynamic_scope(move |scope| {
                        scope.children.lock().unwrap().clear();
                        for #pat in #expr {
                            #(#kids)*
                        }
                    }, #deps_vec_emit);
                }
            }
        }
    }
}

fn emit_node(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Text(text) => quote! {
            scope.view(Arc::new(move |ctx| {
                ctx.draw_text(Point { x: 0, y: 0 }, &format!(#text))
            }));
        },

        RsxNode::Component(name) => quote! {
            scope.child(#name, None);
        },

        RsxNode::Mount(m) => quote! {
            #m.mount();
        },

        RsxNode::If { .. } => panic!("Invalid if statement"),

        RsxNode::For { .. } => panic!("Invalid for loop"),
    }
}
