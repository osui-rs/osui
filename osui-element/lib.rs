use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput, Fields, LifetimeParam};

/// Sets the required values and traits for a struct to be a element.
/// You will still need to implement ElementCore yourself
#[proc_macro_attribute]
pub fn element(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            if ast.generics.lifetimes().count() == 0 {
                ast.generics
                    .params
                    .push(syn::GenericParam::Lifetime(LifetimeParam::new(
                        syn::Lifetime::new("'a", proc_macro2::Span::call_site()),
                    )));
            }

            if let Fields::Named(fields) = &mut struct_data.fields {
                // Example fields, modify according to your needs
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub x: Value<usize> })
                        .unwrap(),
                );
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub y: usize })
                        .unwrap(),
                );
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub width: Value<usize> })
                        .unwrap(),
                );
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub height: Value<usize> })
                        .unwrap(),
                );
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub children: Children })
                        .unwrap(),
                );
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub id: &'a str })
                        .unwrap(),
                );
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub class: &'a str })
                        .unwrap(),
                );
            }
        }
        _ => panic!("`element` can only be used with structs"),
    }

    let expanded_impl = quote! {
        impl<'a> ElementCore for #struct_name<'a> {
            fn get_data(&self) -> (Value<usize>, usize, String) {
                (self.x, self.y, self.id.to_string())
            }
            fn update_data(&mut self, width: usize, height: usize) {
                self.width.try_set_value(width);
                self.height.try_set_value(height);
                if let Children::Children(children, _) = &mut self.children {
                    for child in children {
                        child.update_data(width, height);
                    }
                }
            }
            fn get_element_by_id(&mut self, id: &str) -> Option<&mut Element> {
                if let Children::Children(children, _) = &mut self.children {
                    for elem in children {
                        if elem.get_data().2 == id {
                            return Some(elem);
                        }
                    }
                }
                None
            }
            fn get_child(&mut self) -> Option<&mut Element> {
                if let Children::Children(children, child) = &mut self.children {
                    children.get_mut(*child)
                } else {
                    None
                }
            }
        }
    };

    let expanded = quote! {
        #ast
        #expanded_impl
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn elem_fn(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let func_name = syn::Ident::new(
        &struct_name.to_string().to_lowercase(),
        proc_macro2::Span::call_site(),
    );

    let elem_fn = quote! {
        pub fn #func_name<'a>() -> Box<#struct_name<'a>> {
            Box::new(#struct_name::default())
        }
    };

    let expanded = quote! {
        #ast
        #elem_fn
    };

    expanded.into()
}
