use std::collections::HashSet;

#[proc_macro]
pub fn firestore_path_helper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    format_macro_output(syn::parse_macro_input!(input as MacroInput))
}

enum Segment {
    CollectionId(String),
    DocumentId(DocumentId),
}

enum DocumentId {
    Fixed(String),
    Variable(syn::Ident, syn::Type),
}

struct MacroInput {
    segments: Vec<Segment>,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let format: syn::LitStr = input.parse()?;
        let args = if input.is_empty() {
            Vec::new()
        } else {
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
            syn::punctuated::Punctuated::<Arg, syn::Token![,]>::parse_terminated(input)?
                .into_iter()
                .map(|arg| (arg.ident, arg.typ))
                .collect::<Vec<(syn::Ident, syn::Type)>>()
        };

        let segments = parse_format(format, &args)?;

        Ok(MacroInput { segments })
    }
}

fn format_macro_output(input: MacroInput) -> proc_macro::TokenStream {
    let MacroInput { segments } = input;

    let collection_struct = format_macro_output_collection_struct(&segments);
    let document_struct = format_macro_output_document_struct(&segments);
    let output = quote::quote! {
        #collection_struct
        #document_struct
    };
    proc_macro::TokenStream::from(output)
}

fn format_macro_output_collection_struct(segments: &[Segment]) -> proc_macro2::TokenStream {
    assert!(segments.len() >= 2);
    let segments = segments
        .iter()
        .take(segments.len() - 1)
        .collect::<Vec<&Segment>>();

    let fields = {
        let mut fields = Vec::new();
        let mut seen = HashSet::new();
        for segment in &segments {
            match segment {
                Segment::CollectionId(_) => {
                    // do nothing
                }
                Segment::DocumentId(document_id) => match document_id {
                    DocumentId::Fixed(_) => {
                        // do nothing
                    }
                    DocumentId::Variable(field_name, field_type) => {
                        if seen.insert(field_name.to_string()) {
                            fields.push(quote::quote! {
                                pub #field_name: #field_type
                            });
                        }
                    }
                },
            }
        }
        fields
    };

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
            Segment::DocumentId(document_id) => match document_id {
                DocumentId::Fixed(id) => {
                    quote::quote! {
                        path.push('/');
                        path.push_str(#id);
                    }
                }
                DocumentId::Variable(field_name, _) => {
                    let field =
                        syn::Ident::new(&field_name.to_string(), proc_macro2::Span::call_site());
                    quote::quote! {
                        path.push('/');
                        path.push_str(self.#field.to_string().as_str());
                    }
                }
            },
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

    quote::quote! {
        pub struct Collection {
            #(#fields,)*
        }

        impl Collection {
            #path_method
        }
    }
}

fn format_macro_output_document_struct(segments: &[Segment]) -> proc_macro2::TokenStream {
    let fields = {
        let mut fields = Vec::new();
        let mut seen = HashSet::new();
        for segment in segments {
            match segment {
                Segment::CollectionId(_) => {
                    // do nothing
                }
                Segment::DocumentId(document_id) => match document_id {
                    DocumentId::Fixed(_) => {
                        // do nothing
                    }
                    DocumentId::Variable(field_name, field_type) => {
                        if seen.insert(field_name.to_string()) {
                            fields.push(quote::quote! {
                                pub #field_name: #field_type
                            });
                        }
                    }
                },
            }
        }
        fields
    };

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
            Segment::DocumentId(document_id) => match document_id {
                DocumentId::Fixed(id) => {
                    quote::quote! {
                        path.push('/');
                        path.push_str(#id);
                    }
                }
                DocumentId::Variable(field_name, _) => {
                    let field =
                        syn::Ident::new(&field_name.to_string(), proc_macro2::Span::call_site());
                    quote::quote! {
                        path.push('/');
                        path.push_str(self.#field.to_string().as_str());
                    }
                }
            },
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

    quote::quote! {
        pub struct Document {
            #(#fields,)*
        }

        impl Document {
            #path_method
        }
    }
}

fn parse_format(
    input_format: syn::LitStr,
    input_args: &[(syn::Ident, syn::Type)],
) -> Result<Vec<Segment>, syn::Error> {
    let format = input_format.value();

    let segments = format.split('/').collect::<Vec<&str>>();
    if segments.is_empty() || segments.len() % 2 != 0 {
        return Err(syn::Error::new(
            input_format.span(),
            "format must contain an even number of segments separated by '/'",
        ));
    }

    enum S {
        C(String),
        DF(String),
        DV(String),
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
                    Ok(S::C(segment.to_string()))
                } else {
                    Err(syn::Error::new(
                        input_format.span(),
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
                        Ok(S::DV(field_name.to_owned()))
                    } else {
                        Err(syn::Error::new(
                            input_format.span(),
                            format!("format contains invalid document id segment: '{}'", segment),
                        ))
                    }
                } else {
                    if !segment.is_empty()
                        && segment
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        Ok(S::DF(segment.to_string()))
                    } else {
                        Err(syn::Error::new(
                            input_format.span(),
                            format!("format contains invalid document id segment: '{}'", segment),
                        ))
                    }
                }
            }
        })
        .collect::<Result<Vec<S>, syn::Error>>()?;

    // Check for duplicate document id arguments
    // (duplicate arguments are not allowed)
    let mut seen = HashSet::new();
    for (ident, _) in input_args {
        if !seen.insert(ident.to_string()) {
            return Err(syn::Error::new(
                ident.span(),
                format!("duplicate document id argument: '{}'", ident),
            ));
        }
    }

    // Check that all document id segments match the arguments
    // (missing arguments are not allowed)
    for document_id in segments.iter().filter_map(|it| match it {
        S::C(_) => None,
        S::DF(_) => None,
        S::DV(field_name) => Some(field_name),
    }) {
        if !input_args
            .iter()
            .any(|(ident, _)| &ident.to_string() == document_id)
        {
            return Err(syn::Error::new(
                input_format.span(),
                format!(
                    "document id segment '{}' does not match any argument",
                    document_id
                ),
            ));
        }
    }

    // Check that all arguments match a document id segment
    // (extra arguments are not allowed)
    for (ident, _) in input_args {
        if !segments.iter().any(|segment| match segment {
            S::C(_) => false,
            S::DF(_) => false,
            S::DV(field_name) => field_name == &ident.to_string(),
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

    Ok(segments
        .into_iter()
        .map(|segment| match segment {
            S::C(id) => Segment::CollectionId(id),
            S::DF(id) => Segment::DocumentId(DocumentId::Fixed(id)),
            S::DV(field_name) => input_args
                .iter()
                .find(|(ident, _)| &ident.to_string() == &field_name)
                .cloned()
                .map(|(ident, typ)| Segment::DocumentId(DocumentId::Variable(ident, typ)))
                .expect("document id segment must match an argument"),
        })
        .collect::<Vec<Segment>>())
}
