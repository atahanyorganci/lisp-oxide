extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    ExprAssign, Ident, ItemFn, Token,
};

#[derive(Debug, Default)]
struct BuiltinArgs {
    name: Option<Ident>,
}

impl Parse for BuiltinArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut args = Self::default();
        let vars = Punctuated::<ExprAssign, Token![,]>::parse_terminated(input)?;
        for var in vars {
            match var.left.as_ref() {
                syn::Expr::Path(ep) if ep.path.segments.len() == 1 => {
                    let rhs = &ep.path.segments[0];
                    if rhs.ident == "name" {
                        match var.right.as_ref() {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(s) => {
                                    let ident = Ident::new(&s.value(), s.span());
                                    args.name = Some(ident);
                                }
                                _ => {
                                    return Err(syn::Error::new(
                                        var.right.span(),
                                        "Name attribute expects string value.",
                                    ))
                                }
                            },
                            _ => {
                                return Err(syn::Error::new(
                                    var.right.span(),
                                    "Name attribute expects string value.",
                                ))
                            }
                        }
                    } else {
                        return Err(syn::Error::new(var.left.span(), "Unknown attribute."));
                    }
                }
                _ => return Err(syn::Error::new(var.left.span(), "Unknown attribute.")),
            }
        }

        Ok(args)
    }
}

fn is_env(type_path: &syn::TypePath) -> bool {
    if !is_rc(type_path) {
        return false;
    }
    let env = vec![
        String::from("mal"),
        String::from("env"),
        String::from("Env"),
    ];
    let last = match type_path.path.segments.last() {
        Some(last) => last,
        None => return false,
    };
    if let syn::PathArguments::AngleBracketed(arg) = &last.arguments {
        if let syn::GenericArgument::Type(generic) = &arg.args[0] {
            match generic {
                syn::Type::Path(path) => compare_segments(path, env),
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn is_rc(type_path: &syn::TypePath) -> bool {
    let rc = vec![String::from("std"), String::from("rc"), String::from("Rc")];
    compare_segments(type_path, rc)
}

fn compare_segments(type_path: &syn::TypePath, segments: Vec<String>) -> bool {
    if type_path.path.segments.len() > segments.len() {
        return false;
    }
    let lhs_iter = segments.iter().rev();
    let rhs_iter = type_path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .rev();
    for (lhs, rhs) in lhs_iter.zip(rhs_iter) {
        if lhs.as_str() != rhs.as_str() {
            return false;
        }
    }
    true
}

#[proc_macro_attribute]
pub fn builtin_func(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as BuiltinArgs);
    let func = parse_macro_input!(input as ItemFn);

    // Generated builtin functions name is `mal_{func}`
    let original_name = &func.sig.ident;
    let builtin_name = if let Some(name) = args.name {
        Ident::new(&format!("mal_{}", name), name.span())
    } else {
        Ident::new(&format!("mal_{}", original_name), original_name.span())
    };

    // Gather arg names, and types of the reference from signature
    let mut arg_count = func.sig.inputs.len();
    let mut variadic = false;
    let mut arg_names = Vec::with_capacity(arg_count);
    let mut arg_statements = Vec::with_capacity(arg_count);
    for (index, arg) in func.sig.inputs.iter().enumerate() {
        let pat_type = match arg {
            syn::FnArg::Receiver(_) => {
                return syn::Error::new(arg.span(), "Builtins shouldn't be methods.")
                    .to_compile_error()
                    .into();
            }
            syn::FnArg::Typed(pt) => pt,
        };
        let arg_ident = match pat_type.pat.as_ref() {
            syn::Pat::Ident(pat_ident) => pat_ident.ident.clone(),
            syn::Pat::Wild(_) => Ident::new(format!("_arg{}", index).as_str(), pat_type.pat.span()),
            _ => {
                return syn::Error::new(pat_type.pat.span(), "Unrecognized pattern for argument.")
                    .to_compile_error()
                    .into();
            }
        };
        let arg_type = match pat_type.ty.as_ref() {
            syn::Type::Reference(reference) => reference.elem.as_ref(),
            _ => {
                return syn::Error::new(pat_type.ty.span(), "Builtin function args should be references to types that implement `MalType`, `&Rc<dyn MalType>`, or `&Rc<Env>`.")
                    .to_compile_error()
                    .into();
            }
        };
        let arg_statement = match arg_type {
            syn::Type::Path(ty) if is_env(ty) => {
                arg_count -= 1;
                quote! {
                    let #arg_ident = env;
                }
            }
            syn::Type::Path(ty) if is_rc(ty) => quote! {
                let #arg_ident = &args[#index];
            },
            syn::Type::Path(ty) => quote! {
                let #arg_ident = &args[#index].as_type::<#ty>()?;
            },
            syn::Type::TraitObject(_) => quote! {
                let #arg_ident = args[#index].as_ref();
            },
            syn::Type::Slice(_) => {
                variadic = true;
                quote! {
                    let #arg_ident = &args[#index..];
                }
            }
            _ => {
                return syn::Error::new(pat_type.ty.span(), "Builtin function args should be references to types that implement `MalType`, `&Rc<dyn MalType>`, or `&Rc<Env>`.")
                    .to_compile_error()
                    .into();
            }
        };
        arg_names.push(arg_ident);
        arg_statements.push(arg_statement);
    }

    // Generate the actual call to the original function
    let actual_call = quote! {
        #original_name(#(#arg_names),*)
    };

    // Generate code to check for function argument count
    let arg_count_check = if !variadic {
        quote! {
            if args.len() != #arg_count {
                return std::result::Result::Err(MalError::TypeError);
            }
        }
    } else if arg_count > 1 {
        let actual_arg_count = arg_count - 1;
        quote! {
            if args.len() < #actual_arg_count {
                return std::result::Result::Err(MalError::TypeError);
            }
        }
    } else {
        quote! {}
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
