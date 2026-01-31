//! # OSUI Macros
//!
//! Procedural macros for OSUI that provide ergonomic syntax for defining components.
//!
//! ## Features
//!
//! - `#[component]` - Transforms a function into a reusable component with props
//! - `rsx!` - Creates RSX (React-like Syntax) for component hierarchies

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, Pat, ReturnType, Type, parse_macro_input};

mod emit;
mod parse;

/// RSX (React-like Syntax) macro for building component hierarchies
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Component {
///         prop: value,
///     }
/// }
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as parse::RsxRoot);
    emit::emit_rsx(ast).into()
}

/// Component attribute macro for defining reusable components
///
/// Transforms a function into a component with automatic prop handling.
/// The first parameter must be `cx: &Arc<Context>`.
/// Remaining parameters become component props.
///
/// # Example
///
/// ```rust,ignore
/// #[component]
/// pub fn Counter(cx: &Arc<Context>, initial: &i32) -> View {
///     let count = use_state(*initial);
///     
///     Arc::new(move |ctx| {
///         ctx.draw_text(Point { x: 0, y: 0 }, &format!("Count: {}", count.get_dl()));
///     })
/// }
/// ```
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let vis = &input.vis;
    let body = &input.block;

    let return_ty = match &input.sig.output {
        ReturnType::Type(_, ty) => ty,
        _ => {
            return syn::Error::new_spanned(&input.sig, "component must return View")
                .to_compile_error()
                .into();
        }
    };

    let mut inputs = input.sig.inputs.iter();

    // ---- First param must be cx ----
    let cx = match inputs.next() {
        Some(FnArg::Typed(pat)) => pat,
        _ => {
            return syn::Error::new_spanned(&input.sig, "first argument must be cx: &Arc<Context>")
                .to_compile_error()
                .into();
        }
    };

    let cx_ident = match &*cx.pat {
        Pat::Ident(id) => &id.ident,
        _ => unreachable!(),
    };

    let cx_ty = &cx.ty;

    // ---- Remaining params are props ----
    let mut struct_fields = Vec::new();
    let mut render_params = Vec::new();
    let mut call_args = Vec::new();

    for arg in inputs {
        let FnArg::Typed(pat) = arg else { continue };

        let ident = match &*pat.pat {
            Pat::Ident(id) => &id.ident,
            _ => {
                return syn::Error::new_spanned(pat, "unsupported prop pattern")
                    .to_compile_error()
                    .into();
            }
        };

        // Strip leading &
        let owned_ty = match &*pat.ty {
            Type::Reference(r) => &r.elem,
            ty => ty,
        };

        struct_fields.push(quote! {
            pub #ident: #owned_ty
        });

        render_params.push(quote! {
            #ident: &#owned_ty
        });

        call_args.push(quote! {
            &self.#ident
        });
    }

    let expanded = quote! {
        #vis struct #name {
            #(#struct_fields,)*
        }

        impl #name {
            pub fn component(
                #cx_ident: #cx_ty,
                #(#render_params,)*
            ) -> #return_ty {
                #body
            }
        }

        impl ComponentImpl for #name {
            fn call(&self, cx: &std::sync::Arc<Context>) -> #return_ty {
                Self::component(
                    cx,
                    #(#call_args,)*
                )
            }
        }
    };

    expanded.into()
}
