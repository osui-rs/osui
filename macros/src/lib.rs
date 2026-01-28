use proc_macro::TokenStream;

mod emit;
mod parse;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as parse::RsxRoot);
    emit::emit_rsx(ast).into()
}
