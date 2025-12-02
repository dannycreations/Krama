use proc_macro2::TokenStream;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  parse2,
  punctuated::Punctuated,
  Ident, ItemFn, Lit, LitStr, Token,
};

struct NativeArgs {
  name: LitStr,
  module: LitStr,
}

impl Parse for NativeArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut name = None;
    let mut module = None;

    while !input.is_empty() {
      let ident = input.parse::<Ident>()?;
      input.parse::<Token![=]>()?;
      let value = input.parse::<LitStr>()?;

      if ident == "name" {
        name = Some(value);
      } else if ident == "module" {
        module = Some(value);
      }

      if !input.is_empty() {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(NativeArgs {
      name: name.ok_or_else(|| input.error("missing `name` attribute"))?,
      module: module
        .ok_or_else(|| input.error("missing `module` attribute"))?,
    })
  }
}

struct PropertyArgs {
  name: LitStr,
  types: Punctuated<Lit, Token![,]>,
}

impl Parse for PropertyArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut name = None;
    let mut types = None;

    while !input.is_empty() {
      let ident = input.parse::<Ident>()?;
      input.parse::<Token![=]>()?;

      if ident == "name" {
        name = Some(input.parse::<LitStr>()?);
      } else if ident == "types" {
        let content;
        syn::bracketed!(content in input);
        types = Some(Punctuated::<Lit, Token![,]>::parse_terminated(&content)?);
      }

      if !input.is_empty() {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(PropertyArgs {
      name: name.ok_or_else(|| input.error("missing `name` attribute"))?,
      types: types.ok_or_else(|| input.error("missing `types` attribute"))?,
    })
  }
}

fn transform_fn(
  item_fn: &ItemFn,
  error_msg_prefix: &str,
  inventory_submission: TokenStream,
) -> TokenStream {
  let vis = &item_fn.vis;
  let body = &item_fn.block;
  let sig = &item_fn.sig;

  if sig.asyncness.is_none() {
    return syn::Error::new_spanned(
      sig,
      format!("{} function must be async", error_msg_prefix),
    )
    .to_compile_error();
  }

  // Clone signature and modify it.
  let mut new_sig = sig.clone();
  new_sig.asyncness = None;

  // Check for 'ast lifetime.
  let has_ast_lifetime = new_sig
    .generics
    .lifetimes()
    .any(|lt| lt.lifetime.ident == "ast");
  if !has_ast_lifetime {
    return syn::Error::new_spanned(
      sig,
      format!(
        "{} function must have a `'ast` lifetime parameter",
        error_msg_prefix
      ),
    )
    .to_compile_error();
  }

  new_sig.output = syn::parse_quote! {
      -> futures::future::LocalBoxFuture<'ast, Result<krama_core::object::Object<'ast>, krama_core::error::ErrorKind>>
  };

  quote! {
      #vis #new_sig {
          use futures::future::FutureExt;
          async move #body.boxed_local()
      }

      #inventory_submission
  }
}

fn implement_register_macro<T: Parse>(
  attr: TokenStream,
  item: TokenStream,
  macro_type: &str,
  submission_generator: impl Fn(&T, &Ident) -> TokenStream,
) -> TokenStream {
  let args: T = match parse2(attr) {
    Ok(a) => a,
    Err(e) => return e.to_compile_error(),
  };
  let item_fn: ItemFn = match parse2(item) {
    Ok(f) => f,
    Err(e) => return e.to_compile_error(),
  };

  let fn_name = &item_fn.sig.ident;
  let inventory_submission = submission_generator(&args, fn_name);

  transform_fn(&item_fn, macro_type, inventory_submission)
}

#[proc_macro_attribute]
pub fn register_native(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  implement_register_macro::<NativeArgs>(
    attr.into(),
    item.into(),
    "native",
    |args, fn_name| {
      let name = &args.name;
      let module = &args.module;
      quote! {
          inventory::submit! {
              krama_core::object::StandardNative {
                  name: #name,
                  callback: #fn_name,
                  module: #module,
              }
          }
      }
    },
  )
  .into()
}

#[proc_macro_attribute]
pub fn register_property(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  implement_register_macro::<PropertyArgs>(
    attr.into(),
    item.into(),
    "property",
    |args, fn_name| {
      let name = &args.name;
      let types = &args.types;
      quote! {
          inventory::submit! {
              krama_core::object::StandardProperty {
                  name: #name,
                  callback: #fn_name,
                  types: &[#types],
              }
          }
      }
    },
  )
  .into()
}
