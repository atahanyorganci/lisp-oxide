extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Ident, ItemFn, Lit, Token,
};

enum BuiltinArg {
    Flag(Ident),
    Value(Ident, Lit),
}

impl Parse for BuiltinArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let ident = if lookahead.peek(Ident) {
            input.parse::<Ident>()?
        } else {
            return Err(lookahead.error());
        };
        if input.is_empty() {
            Ok(Self::Flag(ident))
        } else {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![=]) {
                input.parse::<syn::token::Eq>().unwrap();
            } else {
                return Err(lookahead.error());
            }
            let lookahead = input.lookahead1();
            let lit = if lookahead.peek(Lit) {
                input.parse::<syn::Lit>().unwrap()
            } else {
                return Err(lookahead.error());
            };
            Ok(Self::Value(ident, lit))
        }
    }
}

#[derive(Debug)]
struct BuiltinArgs {
    name: Option<Ident>,
    symbol: Option<syn::LitStr>,
    special: bool,
}

impl Default for BuiltinArgs {
    fn default() -> Self {
        Self {
            name: Default::default(),
            symbol: Default::default(),
            special: false,
        }
    }
}

impl Parse for BuiltinArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut args = Self::default();
        let vars = Punctuated::<BuiltinArg, Token![,]>::parse_terminated(input)?;
        for var in vars {
            match &var {
                BuiltinArg::Flag(ident) => {
                    if ident == "special" {
                        args.special = true;
                    } else {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("Unknown flag `{}`.", ident),
                        ));
                    }
                }
                BuiltinArg::Value(ident, lit) => {
                    if ident == "name" {
                        let name = match lit {
                            Lit::Str(s) => Ident::new(&s.value(), s.span()),
                            _ => {
                                return Err(syn::Error::new(
                                    lit.span(),
                                    "Name attribute expects string value.",
                                ))
                            }
                        };
                        args.name = Some(name);
                    } else if ident == "symbol" {
                        let symbol = match lit {
                            Lit::Str(s) => s.clone(),
                            _ => {
                                return Err(syn::Error::new(
                                    lit.span(),
                                    "Symbol attribute expects string value.",
                                ))
                            }
                        };
                        args.symbol = Some(symbol.clone());
                    } else {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("Unknown attribute `{}`.", ident),
                        ));
                    }
                }
            }
        }

        Ok(args)
    }
}

fn is_option(type_path: &syn::TypePath) -> bool {
    let option = vec![
        String::from("std"),
        String::from("option"),
        String::from("Option"),
    ];
    if !compare_segments(type_path, option) {
        return false;
    }
    let last = type_path.path.segments.last().unwrap();
    if let syn::PathArguments::AngleBracketed(arg) = &last.arguments {
        if let syn::GenericArgument::Type(syn::Type::Reference(tr)) = &arg.args[0] {
            if let syn::Type::Path(ty) = tr.elem.as_ref() {
                is_rc_mal_type(ty)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn is_rc_mal_type(type_path: &syn::TypePath) -> bool {
    if !is_rc(type_path) {
        return false;
    }
    let mal_type_path = vec![
        String::from("mal"),
        String::from("types"),
        String::from("MalType"),
    ];
    let last = match type_path.path.segments.last() {
        Some(last) => last,
        None => return false,
    };
    if let syn::PathArguments::AngleBracketed(arg) = &last.arguments {
        if let syn::GenericArgument::Type(syn::Type::TraitObject(tto)) = &arg.args[0] {
            if let syn::TypeParamBound::Trait(tb) = &tto.bounds[0] {
                let lhs_iter = mal_type_path.iter().rev();
                let rhs_iter = tb
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
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
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
        if let syn::GenericArgument::Type(syn::Type::Path(path)) = &arg.args[0] {
            compare_segments(path, env)
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

const TYPE_ERROR_MSG : &str = "Builtin function args should be references to types that implement `MalType`, `&Rc<dyn MalType>`, `Option<&Rc<dyn MalType>>`, or `&Rc<Env>`.";

#[proc_macro_attribute]
pub fn builtin_func(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as BuiltinArgs);
    let func = parse_macro_input!(input as ItemFn);

    // Generated builtin functions name is `mal_{name}`
    // Name value can be set using `name = "renamed"` attribute
    let original_name = &func.sig.ident;
    let name = if let Some(name) = &args.name {
        name
    } else {
        original_name
    };
    let builtin = format!("mal_{}", name);
    let builtin_name = Ident::new(&builtin, name.span());
    let pair_name = Ident::new(&builtin.to_uppercase(), name.span());

    // Symbol for env
    let pair = if !args.special {
        let symbol = if let Some(symbol) = args.symbol {
            symbol
        } else {
            syn::LitStr::new(&name.to_string(), name.span())
        };
        quote! {
            pub const #pair_name: (&'static str, &'static MalFuncPtr) = (#symbol, &#builtin_name);
        }
    } else {
        quote! {}
    };

    // Return type of the builtin function
    let return_type = &func.sig.output;

    // Gather arg names, and types of the reference from signature
    let mut arg_count = func.sig.inputs.len();
    let mut optional_count = 0;
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
        arg_names.push(arg_ident.clone());

        let arg_type = match pat_type.ty.as_ref() {
            syn::Type::Reference(reference) => reference.elem.as_ref(),
            syn::Type::Path(ty) if is_option(ty) => {
                optional_count += 1;
                arg_count -= 1;
                arg_statements.push(quote! {
                    let #arg_ident = args.get(#index);
                });
                continue;
            }
            _ => {
                return syn::Error::new(pat_type.ty.span(), TYPE_ERROR_MSG)
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
                return syn::Error::new(arg_type.span(), TYPE_ERROR_MSG)
                    .to_compile_error()
                    .into();
            }
        };
        arg_statements.push(arg_statement);
    }

    if optional_count != 0 && variadic {
        return syn::Error::new(
            func.sig.inputs.span(),
            "Usage of optional and variadic arguments isn't supported.",
        )
        .to_compile_error()
        .into();
    }

    // Generate the actual call to the original function
    let actual_call = quote! {
        #original_name(#(#arg_names),*)
    };

    // Generate code to check for function argument count
    let arg_count_check = if !variadic && optional_count == 0 {
        quote! {
            if args.len() != #arg_count {
                return std::result::Result::Err(MalError::TypeError);
            }
        }
    } else if variadic && arg_count > 1 {
        // Variadic functions that don't take positional arguments should not generate check
        // since every arg is passed to the function
        let actual_arg_count = arg_count - 1;
        quote! {
            if args.len() < #actual_arg_count {
                return std::result::Result::Err(MalError::TypeError);
            }
        }
    } else if optional_count != 0 {
        let max_count = arg_count + optional_count + 1;
        quote! {
            if !(#arg_count..#max_count).contains(&args.len()) {
                return std::result::Result::Err(MalError::TypeError);
            }
        }
    } else {
        quote! {}
    };

    let generated = quote! {
        #func
        pub fn #builtin_name(args: &[std::rc::Rc<dyn MalType>], env: &std::rc::Rc<Env>) #return_type {
            #arg_count_check
            #(#arg_statements)*
            #actual_call
        }
        #pair
    };
    generated.into()
}
