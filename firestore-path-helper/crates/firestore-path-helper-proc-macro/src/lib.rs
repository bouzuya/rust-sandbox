#[proc_macro]
pub fn firestore_path_helper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as MacroInput);
    format_macro_output(input)
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

fn format_macro_output(input: MacroInput) -> proc_macro::TokenStream {
    // TODO: use `format`
    let MacroInput { format: _, args } = input;

    let fields = args.into_iter().map(|(field_name, field_type)| {
        quote::quote! {
            pub #field_name: #field_type
        }
    });

    let output = quote::quote! {
        pub struct Document {
            #(#fields,)*
        }
    };

    proc_macro::TokenStream::from(output)
}
