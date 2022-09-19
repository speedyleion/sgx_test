extern crate proc_macro;

use syn::{ItemFn, parse};
use proc_macro2::{TokenStream, Span};
use quote::{quote, format_ident};

#[proc_macro_attribute]
pub fn sgx_test(attr: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let input = proc_macro2::TokenStream::from(input);
    let output =
        match sgx_test_impl(attr, input) {
            Ok(ts) => ts,
            Err(e) => e.to_compile_error().into(),
        };

    proc_macro::TokenStream::from(output)
}

/// Generate the necessary wrapper for a test function
fn sgx_test_impl(attr: TokenStream, input: TokenStream) -> parse::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(parse::Error::new(
            Span::call_site(),
            "`#[mytests]` attribute takes no arguments",
        ));
    }
    let function: ItemFn = syn::parse2(input)?;
    let function_name = function.sig.ident.clone();
    let function_string = format!("{}", function_name);
    let test_case_name = format_ident!("__sgx_test_{}", function_name);

    Ok(quote!(
    #[test_case]
    static #test_case_name: TestCase = TestCase{name: #function_string, function: &#function_name};
    #function
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_function() {
        let stream = quote!(
            fn function_1(){
            }
        );
        let stream = sgx_test_impl(TokenStream::new(), stream).unwrap();
        assert_eq!(stream.to_string(), "# [test_case] static __sgx_test_function_1 : TestCase = TestCase { name : \"function_1\" , function : & function_1 } ; fn function_1 () { }");
    }

    #[test]
    fn a_second_function() {
        let stream = quote!(
            fn function_2_with_feeling(){
                assert_eq!(1, 1);
            }
        );
        let stream = sgx_test_impl(TokenStream::new(), stream).unwrap();
        assert_eq!(stream.to_string(), "# [test_case] static __sgx_test_function_2_with_feeling : TestCase = TestCase { name : \"function_2_with_feeling\" , function : & function_2_with_feeling } ; fn function_2_with_feeling () { assert_eq ! (1 , 1) ; }");
    }
}
