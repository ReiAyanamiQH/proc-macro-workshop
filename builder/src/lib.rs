use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    //eprintln!("{:#?}", input);
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let builder_name_literal = format!("{}Builder", struct_name.to_string());
    let builder_name = Ident::new(&builder_name_literal, struct_name.span());
    let fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        Data::Enum(_) | Data::Union(_) => {
            unimplemented!()
        }
    };
    let builder_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            #field_name : std::option::Option<#field_type>,
        }
    });

    let builder_init = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            #field_name : std::option::Option::<_>::None,
        }
    });

    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields)*
        }

        impl #struct_name {
            pub fn builder() -> #builder_name{
                #builder_name {
                    #(#builder_init)*
                }
            }
        }
    };
    TokenStream::from(expanded)
}
