use syn::{DeriveInput, spanned::Spanned as _};

#[proc_macro_derive(EnumStrConv, attributes(enum_str_conv))]
pub fn enum_str_conv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let output = my_derive(input).unwrap_or_else(syn::Error::into_compile_error);
    proc_macro::TokenStream::from(output)
}

fn my_derive(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    Ok(format(parse(input)?))
}

struct Parsed {
    enum_ident: syn::Ident,
    error_ident: syn::Expr,
    unknown_fn: syn::Expr,
    variant_attrs: Vec<(syn::Ident, syn::LitStr)>,
}

fn parse(input: syn::DeriveInput) -> Result<Parsed, syn::Error> {
    let data_enum = if let syn::Data::Enum(data_enum) = &input.data {
        Ok(data_enum)
    } else {
        Err(syn::Error::new_spanned(
            &input,
            "EnumStrConv can only be derived for enums",
        ))
    }?;
    let enum_ident = input.ident.clone();
    let EnumAttr {
        error: error_ident,
        unknown: unknown_fn,
    } = parse_enum_attr(&input)?;
    let variant_attrs = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = variant.ident.clone();
            let VariantAttr { str: variant_str } = parse_variant_attr(&variant)?;
            Ok((variant_ident, variant_str))
        })
        .collect::<Result<Vec<(syn::Ident, syn::LitStr)>, syn::Error>>()?;
    Ok(Parsed {
        enum_ident,
        error_ident,
        unknown_fn,
        variant_attrs,
    })
}

fn format(
    Parsed {
        enum_ident,
        error_ident,
        unknown_fn: unknown,
        variant_attrs,
    }: Parsed,
) -> proc_macro2::TokenStream {
    let display_variants = variant_attrs.iter().map(|(ident, str)| {
        quote::quote! {
            Self::#ident => write!(f, #str)
        }
    });
    let from_str_variants = variant_attrs.iter().map(|(ident, str)| {
        quote::quote! {
            #str => Ok(Self::#ident)
        }
    });
    let output = quote::quote! {
        #[automatically_derived]
        impl ::std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_variants,)*
                }
            }
        }

        #[automatically_derived]
        impl ::std::str::FromStr for #enum_ident {
            type Err = #error_ident;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#from_str_variants,)*
                    _ => Err(#unknown(s.to_owned())),
                }
            }
        }
    };

    output
}

struct EnumAttr {
    error: syn::Expr,
    unknown: syn::Expr,
}

fn parse_enum_attr(input: &DeriveInput) -> Result<EnumAttr, syn::Error> {
    let attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("enum_str_conv"))
        .ok_or_else(|| {
            syn::Error::new_spanned(
                &input,
                "expected attribute: #[enum_str_conv(error = ..., unknown = ...)]",
            )
        })?;
    let nested = attr
        .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .map_err(|_| {
            syn::Error::new_spanned(
                &attr.meta,
                "expected attribute arguments: #[enum_str_conv(error = ..., unknown = ...)]",
            )
        })?;
    let mut error = None;
    let mut unknown = None;
    for meta in nested {
        let meta_name_value = meta.require_name_value()?;
        if meta_name_value.path.is_ident("error") {
            error = Some(meta_name_value.value.clone());
        } else if meta_name_value.path.is_ident("unknown") {
            unknown = Some(meta_name_value.value.clone());
        } else {
            Err(syn::Error::new_spanned(
                &meta_name_value,
                "unknown argument: #[enum_str_conv(error = ..., unknown = ...)]",
            ))?;
        }
    }
    match (error, unknown) {
        (None, None) | (None, Some(_)) => Err(syn::Error::new_spanned(
            &attr.meta,
            "expected `error` argument: #[enum_str_conv(error = ...)]",
        )),
        (Some(_), None) => Err(syn::Error::new_spanned(
            &attr.meta,
            "expected `unknown` argument: #[enum_str_conv(unknown = ...)]",
        )),
        (Some(error), Some(unknown)) => Ok(EnumAttr { error, unknown }),
    }
}

struct VariantAttr {
    str: syn::LitStr,
}

fn parse_variant_attr(variant: &syn::Variant) -> Result<VariantAttr, syn::Error> {
    let attr = variant
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("enum_str_conv"))
        .ok_or_else(|| {
            syn::Error::new(
                variant.span(),
                "expected attribute: #[enum_str_conv(str = ...)]",
            )
        })?;
    let nested = attr
        .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .map_err(|_| {
            syn::Error::new(
                attr.meta.span(),
                "expected attribute arguments: #[enum_str_conv(str = ...)]",
            )
        })?;
    let mut str = None;
    for meta in nested {
        let meta_name_value = meta.require_name_value().unwrap();
        if meta_name_value.path.is_ident("str") {
            match &meta_name_value.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) => {
                    str = Some(lit_str.to_owned());
                }
                _ => {
                    Err(syn::Error::new_spanned(
                        &meta_name_value.value,
                        r#"unknown argument type: #[enum_str_conv(str = "...")]"#,
                    ))?;
                }
            }
        } else {
            Err(syn::Error::new_spanned(
                &meta_name_value,
                "unknown argument: #[enum_str_conv(str = ...)]",
            ))?;
        }
    }
    match str {
        None => Err(syn::Error::new_spanned(
            &attr.meta,
            "expected `str` argument: #[enum_str_conv(str = ...)]",
        )),
        Some(str) => Ok(VariantAttr { str }),
    }
}
