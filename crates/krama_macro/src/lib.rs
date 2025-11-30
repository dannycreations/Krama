use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  parse_macro_input,
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

fn transform_to_boxed_future_fn(
  attr: TokenStream,
  item: TokenStream,
  error_msg_prefix: &str,
) -> TokenStream {
  let item_fn = parse_macro_input!(item as ItemFn);
  let name_attr = parse_macro_input!(attr as NativeArgs);

  let vis = &item_fn.vis;
  let body = &item_fn.block;
  let sig = &item_fn.sig;
  let fn_name = &sig.ident;
  let name = name_attr.name;
  let module = name_attr.module;

  if sig.asyncness.is_none() {
    return syn::Error::new_spanned(
      sig,
      format!("{} function must be async", error_msg_prefix),
    )
    .to_compile_error()
    .into();
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
    .to_compile_error()
    .into();
  }

  new_sig.output = syn::parse_quote! {
      -> futures::future::LocalBoxFuture<'ast, Result<krama_core::object::Object<'ast>, krama_core::error::ErrorKind>>
  };

  let expanded = quote! {
      #vis #new_sig {
          use futures::future::FutureExt;
          async move #body.boxed_local()
      }

      inventory::submit! {
          krama_core::object::StandardNative {
              name: #name,
              callback: #fn_name,
              module: #module,
          }
      }
  };

  TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn register_native(attr: TokenStream, item: TokenStream) -> TokenStream {
  transform_to_boxed_future_fn(attr, item, "native")
}

#[proc_macro_attribute]
pub fn register_property(attr: TokenStream, item: TokenStream) -> TokenStream {
  let item_fn = parse_macro_input!(item as ItemFn);
  let name_attr = parse_macro_input!(attr as PropertyArgs);

  let vis = &item_fn.vis;
  let body = &item_fn.block;
  let sig = &item_fn.sig;
  let fn_name = &sig.ident;
  let name = name_attr.name;
  let types = name_attr.types;

  if sig.asyncness.is_none() {
    return syn::Error::new_spanned(sig, "property function must be async")
      .to_compile_error()
      .into();
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
      "property function must have a `'ast` lifetime parameter",
    )
    .to_compile_error()
    .into();
  }

  new_sig.output = syn::parse_quote! {
      -> futures::future::LocalBoxFuture<'ast, Result<krama_core::object::Object<'ast>, krama_core::error::ErrorKind>>
  };

  let expanded = quote! {
      #vis #new_sig {
          use futures::future::FutureExt;
          async move #body.boxed_local()
      }

      inventory::submit! {
          krama_core::object::StandardProperty {
              name: #name,
              callback: #fn_name,
              types: &[#types],
          }
      }
  };

  TokenStream::from(expanded)
}
