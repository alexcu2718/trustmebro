
use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;
use syn::ItemFn;



#[proc_macro_attribute]
pub fn trustmebro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let vis = input.vis;
    let sig = input.sig;
    let block = input.block;
    
  
    let mut new_sig = sig.clone();
    new_sig.unsafety = None;

    quote! {
        #vis #new_sig {
            unsafe {
                #block
            }
        }
    }.into()
}