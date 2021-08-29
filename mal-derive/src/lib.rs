extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn builtin_func(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    // Generated builtin functions name is `mal_{func}`
    let name = &func.sig.ident;
    let builtin_name = Ident::new(&format!("mal_{}", name), name.span());

    // Gather arg names, and types of the reference from signature
    let arg_count = func.sig.inputs.len();
    let mut arg_names = Vec::with_capacity(arg_count);
    let mut arg_statements = Vec::with_capacity(arg_count);
    for (index, arg) in func.sig.inputs.iter().enumerate() {
        let pat_type = match arg {
            syn::FnArg::Receiver(_) => todo!(),
            syn::FnArg::Typed(pt) => pt,
        };
        let arg_ident = match pat_type.pat.as_ref() {
            syn::Pat::Ident(pat_ident) => pat_ident.ident.clone(),
            _ => todo!(),
        };
        let arg_type = match pat_type.ty.as_ref() {
            syn::Type::Reference(reference) => reference.elem.as_ref(),
            _ => todo!(),
        };
        let arg_statement = quote! {
            let #arg_ident = &args[#index].as_type::<#arg_type>()?;
        };
        arg_names.push(arg_ident);
        arg_statements.push(arg_statement);
    }

    // Generate the actual call to the original function
    let actual_call = quote! {
        #name(#(#arg_names),*)
    };

    // Generate code to check for function argument count
    let arg_count_check = quote! {
        if args.len() != #arg_count {
            return std::result::Result::Err(MalError::TypeError);
        }
    };

    let generated = quote! {
        #func

        pub fn #builtin_name(args: &[std::rc::Rc<dyn MalType>], env: &std::rc::Rc<Env>) -> MalResult {
            #arg_count_check
            #(#arg_statements)*
            #actual_call
        }

    };
    generated.into()
}
