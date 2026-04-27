use proc_macro;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Type,
    parse_macro_input,
};

#[proc_macro_attribute]
pub fn packable(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let buffer_type = parse_macro_input!(args as Type);

    let (pack, unpack, size) = match &input.data {
        Data::Struct(data) => data_struct(data, &buffer_type),
        Data::Enum(data) => data_enum(data, &buffer_type),
        Data::Union(_) => {
            return syn::Error::new_spanned(
                input,
                "Bitpacker can only be derived for structs and enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let stream = quote! {
        #input
        impl ::bitpacker::Packable<#buffer_type> for #ident {
            const SIZE: u32 = #size;
            #[inline]
            fn pack(&self) -> #buffer_type {
                #pack
            }
            #[inline]
            fn unpack(buffer: #buffer_type) -> Self {
                #unpack
            }
        }
    };
    if cfg!(debug_assertions) {
        eprintln!("{}", stream);
    }
    stream.into()
}

fn data_struct(data: &DataStruct, buffer_type: &Type) -> (TokenStream, TokenStream, TokenStream) {
    let FieldsInfo {
        pack,
        unpack,
        bracketed_idents,
        size,
    } = match &data.fields {
        Fields::Named(fields) => fields_named(buffer_type, fields),
        Fields::Unnamed(fields) => fields_unnamed(buffer_type, fields),
        Fields::Unit => {
            return (
                quote! { <#buffer_type as ::bitpacker::Buffer>::ZERO },
                quote! { Self },
                quote! { 0 },
            );
        }
    };

    (
        quote! {
            let mut packer = ::bitpacker::Packer::new();
            let Self #bracketed_idents = self;
            #pack
            packer.into_inner()
        },
        quote! {
            let mut unpacker = ::bitpacker::Unpacker::new(buffer);
            #unpack
            Self #bracketed_idents
        },
        size,
    )
}

fn data_enum(data: &DataEnum, buffer_type: &Type) -> (TokenStream, TokenStream, TokenStream) {
    let variant_len = data.variants.len();
    let variant_size = if variant_len == 1 {
        0
    } else {
        (variant_len - 1).ilog2() + 1
    };
    let mut pack_variants = Vec::new();
    let mut unpack_variants = Vec::new();
    let mut each_variant_size = Vec::new();

    for (i, variant) in data.variants.iter().enumerate() {
        let FieldsInfo {
            pack,
            unpack,
            bracketed_idents,
            size,
        } = match &variant.fields {
            Fields::Named(fields) => fields_named(buffer_type, fields),
            Fields::Unnamed(fields) => fields_unnamed(buffer_type, fields),
            Fields::Unit => FieldsInfo {
                pack: quote! {},
                unpack: quote! {},
                bracketed_idents: quote! {},
                size: quote! { 0 },
            },
        };

        let index = i as u32;
        let ident = &variant.ident;

        pack_variants.push(quote! {
            Self::#ident #bracketed_idents => {
                #pack
                packer.raw_pack(#index as #buffer_type, #variant_size);
            }
        });
        unpack_variants.push(quote! {
            #index => {
                #unpack
                Self::#ident #bracketed_idents
            }
        });
        each_variant_size.push(size);
    }

    let pack = quote! {
        let mut packer = ::bitpacker::Packer::new();
        match self {
            #(#pack_variants)*
        }
        packer.into_inner()
    };
    let unpack = quote! {
        let mut unpacker = ::bitpacker::Unpacker::new(buffer);
        let variant_index = unpacker.raw_unpack(#variant_size) as u32;
        match variant_index {
            #(#unpack_variants)*
            _ => panic!("Invalid variant index"),
        }
    };
    let size = quote! {
        #variant_size + {
            let mut size = 0u32;
            #(
                let s = #each_variant_size;
                if s > size { size = s; }
            )*
            size
        }
    };

    (pack, unpack, size)
}

struct FieldsInfo {
    pack: TokenStream,
    unpack: TokenStream,
    bracketed_idents: TokenStream,
    size: TokenStream,
}

fn fields_named(buffer_type: &Type, fields: &FieldsNamed) -> FieldsInfo {
    let mut idents = Vec::new();
    let mut packs = Vec::new();
    let mut unpacks = Vec::new();
    let mut sizes = Vec::new();
    for field in &fields.named {
        let ident = field.ident.as_ref().unwrap();
        idents.push(quote! { #ident });
        packs.push(quote! { packer.pack(#ident); });
        let ty = &field.ty;
        unpacks.push(quote! { let #ident: #ty = unpacker.unpack(); });
        sizes.push(quote! {
            <#ty as ::bitpacker::Packable<#buffer_type>>::SIZE
        });
    }
    let unpacks = unpacks.into_iter().rev();
    FieldsInfo {
        pack: quote! { #(#packs)* },
        unpack: quote! { #(#unpacks)* },
        bracketed_idents: quote! { { #(#idents),* } },
        size: quote! { 0#(+ #sizes)* },
    }
}

fn fields_unnamed(buffer_type: &Type, fields: &FieldsUnnamed) -> FieldsInfo {
    let mut idents = Vec::new();
    let mut packs = Vec::new();
    let mut unpacks = Vec::new();
    let mut sizes = Vec::new();
    for (i, field) in fields.unnamed.iter().enumerate() {
        let ident = format_ident!("__bit_packer_field{}", i);
        idents.push(quote! { #ident });
        packs.push(quote! { packer.pack(#ident); });
        let ty = &field.ty;
        unpacks.push(quote! { let #ident: #ty = unpacker.unpack(); });
        sizes.push(quote! {
            <#ty as ::bitpacker::Packable<#buffer_type>>::SIZE
        });
    }
    let unpacks = unpacks.into_iter().rev();
    FieldsInfo {
        pack: quote! { #(#packs)* },
        unpack: quote! { #(#unpacks)* },
        bracketed_idents: quote! { (#(#idents),*) },
        size: quote! { 0#(+ #sizes)* },
    }
}
