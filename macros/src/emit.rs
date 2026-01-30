use crate::parse::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn emit_rsx(root: RsxRoot) -> TokenStream {
    emit_rsx_vec(&root.nodes)
}

pub fn emit_rsx_vec(nodes: &Vec<RsxNode>) -> TokenStream {
    let nodes = nodes.iter().map(emit_node_scope);

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

        quote! {
            std::sync::Arc::new(#ident) as std::sync::Arc<dyn HookDependency>
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
                r.static_scope(move |scope| {#emit});
            }
        }

        RsxNode::Component { .. } => {
            let emit = emit_node(node);
            quote! {
                r.static_scope(move |scope| {#emit});
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

        RsxNode::Expr(expr) => quote! {
            r.child(#expr);
        },
    }
}

fn emit_node(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Text(text) => quote! {
            scope.view(Arc::new(move |ctx| {
                ctx.draw_text(Point { x: 0, y: 0 }, &format!(#text))
            }));
        },

        RsxNode::Component {
            path,
            props,
            children,
        } => {
            let prop_inits = props.iter().map(|p| {
                let name = &p.name;
                let value = &p.value;
                quote! { #name: #value }
            });

            let emit_children = emit_rsx_vec(children);

            let component_expr = if children.len() > 0 {
                quote! {
                    #path {
                        #(#prop_inits,)*
                        children: #emit_children
                    }
                }
            } else {
                quote! {
                    #path {
                        #(#prop_inits,)*
                    }
                }
            };

            quote! {
                scope.child(#component_expr, None);
            }
        }

        RsxNode::Mount(m) => quote! {
            #m.mount();
        },

        RsxNode::If { .. } => panic!("Invalid if statement"),

        RsxNode::For { .. } => panic!("Invalid for loop"),

        RsxNode::Expr(expr) => quote! {
            r.child(#expr);
        },
    }
}
