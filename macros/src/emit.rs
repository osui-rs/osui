//! # RSX Emission
//!
//! Converts parsed RSX AST into Rust code that constructs RSX objects.

use crate::parse::*;
use proc_macro2::TokenStream;
use quote::quote;

/// Emits code for the root RSX
pub fn emit_rsx(root: RsxRoot) -> TokenStream {
    emit_rsx_vec(&root.nodes)
}

/// Emits code for a vector of RSX nodes
pub fn emit_rsx_vec(nodes: &Vec<RsxNode>) -> TokenStream {
    let nodes = nodes.iter().map(emit_node_scope);

    quote! {{
        let mut r = osui::frontend::Rsx::new();
        #(#nodes)*
        r
    }}
}

/// Emits variable bindings for dependencies
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

/// Emits a Vec of dependencies as HookDependency trait objects
fn emit_deps_vec(deps: &[Dep]) -> TokenStream {
    let deps = deps.iter().filter(|d| d.is_dep).map(|d| {
        let ident = &d.ident;

        quote! {
            std::sync::Arc::new(#ident) as std::sync::Arc<dyn HookDependency>
        }
    });

    quote! {
        vec![ #(#deps),* ]
    }
}

/// Emits a Vec of plugins
fn emit_plugins(deps: &[ViewPlugin]) -> TokenStream {
    deps.iter()
        .map(|d| {
            let path = &d.path;
            if let Some(args) = &d.args {
                quote!( #path(ctx, &view #(,#args)*); )
            } else {
                quote!( #path(ctx, &view); )
            }
        })
        .collect()
}

/// Emits code for a single node within a scope
fn emit_node_scope(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Text { deps, .. } => {
            let emit = emit_node(node);
            let deps_emit = emit_deps(deps);

            quote! {
                #deps_emit
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
        RsxNode::Text { text, deps } => {
            let deps_emit = emit_deps(deps);

            quote! {
                scope.view(Arc::new({
                    #deps_emit
                    move |ctx| ctx.draw_text(Point { x: 0, y: 0 }, &format!(#text))
                }));
            }
        }

        RsxNode::Component {
            plugins,
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

            if plugins.len() == 0 {
                quote! {
                    scope.child(#component_expr, None);
                }
            } else {
                let plugins_emit = emit_plugins(plugins);

                quote! {
                    scope.child(#component_expr, Some(std::sync::Arc::new(|ctx, view| {
                        let area = ctx.allocate(ctx.area.x, ctx.area.y, ctx.area.width, ctx.area.height);
                        ctx.draw_view(area, view.clone());
                        #plugins_emit
                    })));
                }
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
