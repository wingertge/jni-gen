extern crate proc_macro;

use proc_macro2::{TokenStream, Ident, Span};
use syn::{ItemImpl, ImplItem, ImplItemMethod, Visibility, Type, FnArg, ReturnType};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn java_class(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = attr.to_string();
    let package = args.trim_matches(|c| c == ')' || c == '(').trim().replace("\"", "");
    let impl_block = syn::parse_macro_input!(item as ItemImpl);
    let self_type = &*impl_block.self_ty;
    let class_name = self_type.to_token_stream().to_string();
    let qualified_class_name = format!("Java_{}_{}", package.replace(".", "_"), class_name);
    let methods = impl_block.items.iter().filter_map(|item| {
        match item {
            ImplItem::Method(method) => Some(method),
            _ => None
        }
    }).filter(|method| match method.vis {
        Visibility::Public(_) => true,
        _ => false
    }).map(|method| generate_method_mapping(method, &qualified_class_name, self_type));

    let result = quote! {
        #impl_block

        #(#methods) *
    };

    println!("{}", result.to_string());

    result.into()
}

fn generate_method_mapping(method: &ImplItemMethod, qualified_class_name: &str, self_type: &Type) -> TokenStream {
    let method_name = &method.sig.ident;
    let full_name = format!("{}_{}", qualified_class_name, method.sig.ident);
    let ident = Ident::new(&full_name, Span::call_site());
    let args = &method.sig.inputs;
    let output = match &method.sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, type_) => quote!(#type_)
    };

    let arg_idents = args.iter().map(|arg| match arg {
        FnArg::Receiver(_) => quote!(self),
        FnArg::Typed(pat) => pat.pat.to_token_stream()
    });

    quote! {
        #[no_mangle]
        pub extern "system" fn #ident(#args) -> #output {
            #self_type::#method_name(#(#arg_idents), *)
        }
    }
}