use std::collections::HashSet;

#[proc_macro]
pub fn firestore_path_helper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as MacroInput);
    let segments = match validate_macro_input(&input) {
        Ok(segments) => segments,
        Err(e) => return e.to_compile_error().into(),
    };
    format_macro_output(input, segments)
}

struct MacroInput {
    format: syn::LitStr,
    args: Vec<(syn::Ident, syn::Type)>,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let format: syn::LitStr = input.parse()?;
        let _comma: syn::Token![,] = input.parse()?;

        struct Arg {
            ident: syn::Ident,
            _eq: syn::Token![=],
            typ: syn::Type,
        }
        impl syn::parse::Parse for Arg {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let ident: syn::Ident = input.parse()?;
                let _eq: syn::Token![=] = input.parse()?;
                let typ: syn::Type = input.parse()?;
                Ok(Arg { ident, _eq, typ })
            }
        }
        let args = syn::punctuated::Punctuated::<Arg, syn::Token![,]>::parse_terminated(input)?
            .into_iter()
            .map(|arg| (arg.ident, arg.typ))
            .collect::<Vec<(syn::Ident, syn::Type)>>();

        Ok(MacroInput { format, args })
    }
}

fn format_macro_output(input: MacroInput, segments: Vec<Segment>) -> proc_macro::TokenStream {
    let MacroInput { format: _, args } = input;

    let fields = args.into_iter().map(|(field_name, field_type)| {
        quote::quote! {
            pub #field_name: #field_type
        }
    });

    let path_method = {
        let push_first_segment = match segments.first() {
            Some(Segment::CollectionId(id)) => quote::quote! {
                path.push_str(#id);
            },
            Some(Segment::DocumentId(_)) | None => unreachable!(),
        };
        let push_other_segments = segments.iter().skip(1).map(|segment| match segment {
            Segment::CollectionId(id) => quote::quote! {
                path.push('/');
                path.push_str(#id);
            },
            Segment::DocumentId(field_name) => {
                let field = syn::Ident::new(&field_name, proc_macro2::Span::call_site());
                quote::quote! {
                    path.push('/');
                    path.push_str(self.#field.to_string().as_str());
                }
            }
        });
        quote::quote! {
            pub fn path(&self) -> ::std::string::String {
                let mut path = ::std::string::String::new();
                #push_first_segment
                #(#push_other_segments)*
                path
            }
        }
    };

    let output = quote::quote! {
        pub struct Document {
            #(#fields,)*
        }

        impl Document {
            #path_method
        }
    };

    proc_macro::TokenStream::from(output)
}

enum Segment {
    CollectionId(String),
    DocumentId(String),
}

fn validate_macro_input(input: &MacroInput) -> Result<Vec<Segment>, syn::Error> {
    let format = input.format.value();

    let segments = format.split('/').collect::<Vec<&str>>();
    if segments.is_empty() || segments.len() % 2 != 0 {
        return Err(syn::Error::new(
            input.format.span(),
            "format must contain an even number of segments separated by '/'",
        ));
    }

    let segments = segments
        .iter()
        .enumerate()
        .map(|(i, segment)| (i % 2 == 0, segment))
        .map(|(is_collection_id, segment)| {
            if is_collection_id {
                if !segment.is_empty()
                    && segment
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    Ok(Segment::CollectionId(segment.to_string()))
                } else {
                    Err(syn::Error::new(
                        input.format.span(),
                        format!(
                            "format contains invalid collection id segment: '{}'",
                            segment
                        ),
                    ))
                }
            } else {
                if segment.starts_with('{') && segment.ends_with('}') {
                    let field_name = &segment[1..segment.len() - 1]; // remove '{' and '}'
                    if !field_name.is_empty()
                        && !field_name
                            .chars()
                            .next()
                            .map(|c| c.is_ascii_digit())
                            .unwrap_or(false)
                        && field_name
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        Ok(Segment::DocumentId(field_name.to_owned()))
                    } else {
                        Err(syn::Error::new(
                            input.format.span(),
                            format!("format contains invalid document id segment: '{}'", segment),
                        ))
                    }
                } else {
                    Err(syn::Error::new(
                        input.format.span(),
                        format!("format contains invalid document id segment: '{}'", segment),
                    ))
                }
            }
        })
        .collect::<Result<Vec<Segment>, syn::Error>>()?;

    // Check for duplicate document id arguments
    let mut seen = HashSet::new();
    for (ident, _) in &input.args {
        if !seen.insert(ident.to_string()) {
            return Err(syn::Error::new(
                ident.span(),
                format!("duplicate document id argument: '{}'", ident),
            ));
        }
    }

    // Check that all document id segments match the arguments
    for document_id in segments.iter().filter_map(|it| match it {
        Segment::CollectionId(_) => None,
        Segment::DocumentId(field_name) => Some(field_name),
    }) {
        if !input
            .args
            .iter()
            .any(|(ident, _)| &ident.to_string() == document_id)
        {
            return Err(syn::Error::new(
                input.format.span(),
                format!(
                    "document id segment '{}' does not match any argument",
                    document_id
                ),
            ));
        }
    }
    // ??? TODO: Allow empty args ???
    // ```rust
    // firestore_path_helper!("col1/doc1");
    // assert_eq!(Document {}.path(), "col1/doc1");
    // ```
    assert!(!input.args.is_empty());

    // Check that all arguments match a document id segment
    for (ident, _) in &input.args {
        if !segments.iter().any(|segment| match segment {
            Segment::CollectionId(_) => false,
            Segment::DocumentId(field_name) => field_name == &ident.to_string(),
        }) {
            return Err(syn::Error::new(
                ident.span(),
                format!(
                    "argument '{}' does not match any document id segment",
                    ident
                ),
            ));
        }
    }

    Ok(segments)
}
