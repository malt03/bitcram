use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Index,
    Type, parse_macro_input,
};

#[proc_macro_attribute]
pub fn packable(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let buffer_type = parse_macro_input!(args as Type);

    let (packs, size) = match &input.data {
        Data::Struct(data) => data_struct(data, &buffer_type),
        Data::Enum(data) => data_enum(ident, data, &buffer_type),
        _ => panic!("Packable can only be derived for structs or enums"),
    };

    input.attrs.iter().for_each(|attr| {
        eprintln!("Attribute: {:?}", attr.path().get_ident());
    });

    let stream = quote! {
        #input
        impl ::bitpacker::Packable<#buffer_type> for #ident {
            const SIZE: u32 = #size;
            fn pack(&self) -> #buffer_type {
                let mut packer = ::bitpacker::Packer::new();
                #packs
                packer.into_inner()
            }
            fn unpack(buffer: #buffer_type) -> Self {
                unimplemented!()
            }
        }
    };
    eprintln!("{}", stream);
    stream.into()
}

fn data_struct(
    data: &DataStruct,
    buffer_type: &Type,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut packs = Vec::new();
    let mut size = Vec::new();
    for (i, field) in data.fields.iter().enumerate() {
        let ident = match &field.ident {
            Some(ident) => quote! { #ident },
            None => {
                let index = Index::from(i);
                quote! { #index }
            }
        };
        packs.push(quote! {
            packer.pack(&self.#ident);
        });
        let ty = &field.ty;
        size.push(quote! {
            <#ty as ::bitpacker::Packable<#buffer_type>>::SIZE
        });
    }
    (quote! { #(#packs)* }, quote! { 0#(+ #size)* })
}

fn data_enum(
    enum_ident: &Ident,
    data: &DataEnum,
    buffer_type: &Type,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let variant_len = data.variants.len();
    let variant_size = if variant_len == 0 {
        0
    } else {
        (variant_len - 1).ilog2() + 1
    };
    let mut pack_variants = Vec::new();
    let mut each_variant_size = Vec::new();

    for (i, variant) in data.variants.iter().enumerate() {
        let index = Index::from(i);
        let ident = &variant.ident;
        let pack_variant = quote! {
            packer.raw_pack(#index, #variant_size);
        };

        match &variant.fields {
            Fields::Named(fields) => {
                let (pack, size) =
                    fields_named(enum_ident, buffer_type, ident, &pack_variant, fields);
                pack_variants.push(pack);
                each_variant_size.push(size);
            }
            Fields::Unnamed(fields) => {
                let (pack, size) =
                    fields_unnamed(enum_ident, buffer_type, ident, &pack_variant, fields);
                pack_variants.push(pack);
                each_variant_size.push(size);
            }
            Fields::Unit => {
                pack_variants.push(quote! {
                    #enum_ident::#ident => { #pack_variant }
                });
            }
        };
    }

    let pack = quote! {
        match self {
            #(#pack_variants)*
        }
    };

    let size = quote! {
        #variant_size + {
            let mut size = 0u32;
            #(
                if #each_variant_size > size { size = #each_variant_size; }
            )*
            size
        }
    };

    (pack, size)
}

fn fields_named(
    enum_ident: &Ident,
    buffer_type: &Type,
    variant_ident: &Ident,
    pack_variant: &proc_macro2::TokenStream,
    fields: &FieldsNamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut field_idents = Vec::new();
    let mut field_packs = Vec::new();
    let mut field_sizes = Vec::new();
    for field in &fields.named {
        let ident = field.ident.as_ref().unwrap();
        field_idents.push(quote! { #ident });
        field_packs.push(quote! { packer.pack(#ident); });
        let ty = &field.ty;
        field_sizes.push(quote! {
            <#ty as ::bitpacker::Packable<#buffer_type>>::SIZE
        });
    }
    let field_idents = Itertools::intersperse(field_idents.into_iter(), quote! {, });
    (
        quote! {
            #enum_ident::#variant_ident { #(#field_idents)* } => {
                #pack_variant
                #(#field_packs)*
            }
        },
        quote! { 0#(+ #field_sizes)* },
    )
}

fn fields_unnamed(
    enum_ident: &Ident,
    buffer_type: &Type,
    variant_ident: &Ident,
    pack_variant: &proc_macro2::TokenStream,
    fields: &FieldsUnnamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut field_idents = Vec::new();
    let mut field_packs = Vec::new();
    let mut field_sizes = Vec::new();
    for (i, field) in fields.unnamed.iter().enumerate() {
        let ident = format_ident!("field{}", i);
        field_idents.push(quote! { #ident });
        field_packs.push(quote! { packer.pack(#ident); });
        let ty = &field.ty;
        field_sizes.push(quote! {
            <#ty as ::bitpacker::Packable<#buffer_type>>::SIZE
        });
    }
    let field_idents = Itertools::intersperse(field_idents.into_iter(), quote! {, });
    (
        quote! {
            #enum_ident::#variant_ident(#(#field_idents)*) => {
                #pack_variant
                #(#field_packs)*
            }
        },
        quote! { 0#(+ #field_sizes)* },
    )
}
