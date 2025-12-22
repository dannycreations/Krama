use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  parse2,
  punctuated::Punctuated,
  Error as SynError, Ident, ItemFn, Lit, LitStr, Token,
};

struct GlobalArgs {
  name: LitStr,
}

impl Parse for GlobalArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let name = input.parse::<LitStr>()?;
    Ok(GlobalArgs { name })
  }
}

struct ModuleArgs {
  name: LitStr,
  module: LitStr,
}

impl Parse for ModuleArgs {
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

    Ok(ModuleArgs {
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
  linkme_submission: TokenStream2,
) -> TokenStream2 {
  let vis = &item_fn.vis;
  let body = &item_fn.block;
  let sig = &item_fn.sig;

  if sig.asyncness.is_none() {
    return SynError::new_spanned(
      sig,
      format!("{} function must be async", error_msg_prefix),
    )
    .to_compile_error();
  }

  let mut new_sig = sig.clone();
  new_sig.asyncness = None;

  let has_ast_lifetime = new_sig
    .generics
    .lifetimes()
    .any(|lt| lt.lifetime.ident == "ast");
  if !has_ast_lifetime {
    return SynError::new_spanned(
      sig,
      format!(
        "{} function must have a `'ast` lifetime parameter",
        error_msg_prefix
      ),
    )
    .to_compile_error();
  }

  new_sig.output = syn::parse_quote! {
    -> futures::future::LocalBoxFuture<'ast, Result<krama_core::ObjectKind<'ast>, krama_core::ErrorKind>>
  };

  quote! {
    #vis #new_sig {
      use futures::future::FutureExt;
      async move #body.boxed_local()
    }

    #linkme_submission
  }
}

fn implement_register_macro<T: Parse>(
  attr: TokenStream2,
  item: TokenStream2,
  macro_type: &str,
  submission_generator: impl Fn(&T, &Ident) -> TokenStream2,
) -> TokenStream2 {
  let args: T = match parse2(attr) {
    Ok(a) => a,
    Err(e) => return e.to_compile_error(),
  };
  let item_fn: ItemFn = match parse2(item) {
    Ok(f) => f,
    Err(e) => return e.to_compile_error(),
  };

  let fn_name = &item_fn.sig.ident;
  let linkme_submission = submission_generator(&args, fn_name);

  transform_fn(&item_fn, macro_type, linkme_submission)
}

#[proc_macro_attribute]
pub fn register_global(attr: TokenStream, item: TokenStream) -> TokenStream {
  implement_register_macro::<GlobalArgs>(
    attr.into(),
    item.into(),
    "global",
    |args, fn_name| {
      let name = &args.name;
      let static_name = quote::format_ident!(
        "__KRAMA_GLOBAL_{}",
        fn_name.to_string().to_uppercase()
      );
      quote! {
        #[linkme::distributed_slice(krama_core::STANDARD_GLOBALS)]
        #[allow(non_upper_case_globals)]
        static #static_name: krama_core::StandardGlobal = krama_core::StandardGlobal {
          name: #name,
          callback: #fn_name,
        };
      }
    },
  )
  .into()
}

#[proc_macro_attribute]
pub fn register_module(attr: TokenStream, item: TokenStream) -> TokenStream {
  implement_register_macro::<ModuleArgs>(
    attr.into(),
    item.into(),
    "module",
    |args, fn_name| {
      let name = &args.name;
      let module = &args.module;
      let static_name = quote::format_ident!(
        "__KRAMA_MODULE_{}",
        fn_name.to_string().to_uppercase()
      );
      quote! {
        #[linkme::distributed_slice(krama_core::STANDARD_MODULES)]
        #[allow(non_upper_case_globals)]
        static #static_name: krama_core::StandardModule = krama_core::StandardModule {
          name: #name,
          callback: #fn_name,
          module: #module,
        };
      }
    },
  )
  .into()
}

#[proc_macro_attribute]
pub fn register_property(attr: TokenStream, item: TokenStream) -> TokenStream {
  implement_register_macro::<PropertyArgs>(
    attr.into(),
    item.into(),
    "property",
    |args, fn_name| {
      let name = &args.name;
      let types = &args.types;
      let static_name = quote::format_ident!(
        "__KRAMA_PROPERTY_{}",
        fn_name.to_string().to_uppercase()
      );
      quote! {
        #[linkme::distributed_slice(krama_core::STANDARD_PROPERTIES)]
        #[allow(non_upper_case_globals)]
        static #static_name: krama_core::StandardProperty = krama_core::StandardProperty {
          name: #name,
          callback: #fn_name,
          types: &[#types],
        };
      }
    },
  )
  .into()
}
