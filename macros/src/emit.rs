use crate::parse::*;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub fn emit_rsx(root: RsxRoot) -> TokenStream {
    let nodes = root.nodes.iter().map(emit_node);

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

fn emit_node(node: &RsxNode) -> TokenStream {
    match node {
        RsxNode::Text(text) => quote! {
            {
                let scope = osui::component::Scope::new();
                scope.view(Arc::new(move |ctx| {
                    ctx.draw_text(Point { x: 0, y: 0 }, &format!(#text))
                }));
                r.static_scope(scope);
            }
        },

        RsxNode::Component(name) => quote! {
            {
                let scope = osui::component::Scope::new();
                scope.child(#name, None);
                r.static_scope(scope);
            }
        },

        RsxNode::Mount(m) => quote! {
            #m.mount();
        },

        RsxNode::If {
            deps,
            cond,
            children,
        } => {
            let deps_emit = emit_deps(deps);
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
                    });
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
            let kids = children.iter().map(emit_node);

            quote! {
                {
                    #deps_emit
                    r.dynamic_scope(move |scope| {
                        scope.children.lock().unwrap().clear();
                        for #pat in #expr {
                            #(#kids)*
                        }
                    });
                }
            }
        }
    }
}
