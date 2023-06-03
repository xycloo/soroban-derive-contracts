use proc_macro::TokenStream;
use simple_token::impl_simple_token_macro;

mod simple_token;

#[proc_macro_derive(SimpleTokenMacro)]
pub fn simple_token_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_simple_token_macro(&ast)
}
