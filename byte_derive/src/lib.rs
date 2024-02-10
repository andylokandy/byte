use syn::{punctuated::Punctuated, spanned::Spanned};

struct FieldAttributes {
    read_ctx: Option<syn::Expr>,
    write_ctx: Option<syn::Expr>,
}

fn impl_struct_read(
    name: &syn::Ident,
    struct_fields: &syn::Fields,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    let mut lifetimes = generics.lifetimes();
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let mut generics = generics.clone();

    // use the first lifetime parameter as the input lifetime if there is one
    // otherwise create a new lifetime parameter and add it to the generics
    let input_lt = if let Some(lt) = lifetimes.next() {
        lt.clone()
    } else {
        let lt = syn::LifetimeParam::new(syn::Lifetime::new("'input", generics.span()));
        generics
            .params
            .push_value(syn::GenericParam::Lifetime(lt.clone()));
        generics.params.push_punct(syn::Token![,](generics.span()));
        lt
    };
    let (impl_generics, _, _) = generics.split_for_impl();

    let fields = match struct_fields {
        syn::Fields::Named(fields) => &fields.named,
        syn::Fields::Unnamed(fields) => &fields.unnamed,
        syn::Fields::Unit => {
            // no fields, so no bytes to read
            return quote::quote! {
                impl #impl_generics ::byte::TryRead<#input_lt, ::byte::ctx::Endian> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_read(bytes: & #input_lt [u8], ctx: ::byte::ctx::Endian) -> ::byte::Result<(Self, usize)> {
                        Ok((Self, 0))
                    }
                }
            };
        }
    };

    if lifetimes.next().is_some() {
        return syn::Error::new(
            generics.span(),
            "only one lifetime parameter is allowed for TryRead derive",
        )
        .to_compile_error();
    }

    let field_names = fields.iter().enumerate().map(|(i, field)| {
        field
            .ident
            .clone()
            .unwrap_or_else(|| syn::Ident::new(&format!("_{i}"), field.span()))
    });

    let field_reads = fields.iter().zip(field_names.clone()).map(|(field, name)| {
        let attr = field.attrs.iter().find(|attr| attr.path().is_ident("byte"));
        if let Some(attr) = attr {
            match parse_field_attrs(attr) {
                Ok(attrs) => {
                    let read_ctx = attrs
                        .read_ctx
                        .map_or_else(|| quote::quote!(ctx), |ctx| quote::quote!(#ctx));
                    quote::quote!(let #name = ::byte::BytesExt::read_with(bytes, offset, #read_ctx)?;)
                }
                Err(err) => err.to_compile_error(),
            }
        } else {
            quote::quote!(let #name = ::byte::BytesExt::read_with(bytes, offset, ctx)?;)
        }
    });

    let result = match struct_fields {
        syn::Fields::Named(_) => quote::quote!(Self { #(#field_names),* }),
        syn::Fields::Unnamed(_) => quote::quote!(Self( #(#field_names),* )),
        syn::Fields::Unit => unreachable!(),
    };

    quote::quote! {
        impl #impl_generics ::byte::TryRead<#input_lt, ::byte::ctx::Endian> for #name #ty_generics #where_clause {
            fn try_read(bytes: & #input_lt [u8], ctx: ::byte::ctx::Endian) -> ::byte::Result<(Self, usize)> {
                let mut offset = &mut 0;
                #(#field_reads)*
                Ok((#result, *offset))
            }
        }
    }
}

fn impl_struct_write(
    name: &syn::Ident,
    struct_fields: &syn::Fields,
    generics: &syn::Generics,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match struct_fields {
        syn::Fields::Named(fields) => &fields.named,
        syn::Fields::Unnamed(fields) => &fields.unnamed,
        syn::Fields::Unit => {
            // no fields, so no bytes to read
            return quote::quote! {
                impl #impl_generics ::byte::TryWrite<::byte::ctx::Endian> for #name #ty_generics #where_clause {
                    #[inline]
                    fn try_write(self, bytes: &mut [u8], ctx: ::byte::ctx::Endian) -> ::byte::Result<usize> {
                        Ok(0)
                    }
                }
            };
        }
    };

    let field_names = fields.iter().enumerate().map(|(i, field)| {
        field
            .ident
            .clone()
            .unwrap_or_else(|| syn::Ident::new(&format!("_{i}"), field.span()))
    });

    let field_writes = fields.iter().zip(field_names.clone()).map(|(field, name)| {
        if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("byte")) {
            match parse_field_attrs(attr) {
                Ok(value) => {
                    let write_ctx = value
                        .write_ctx
                        .map_or_else(|| quote::quote!(ctx), |ctx| quote::quote!(#ctx));
                    quote::quote!(::byte::BytesExt::write_with(bytes, offset, #name, #write_ctx)?;)
                }
                Err(err) => err.to_compile_error(),
            }
        } else {
            quote::quote!(::byte::BytesExt::write_with(bytes, offset, #name, ctx)?;)
        }
    });

    let extract_fields = match struct_fields {
        syn::Fields::Named(_) => Some(quote::quote! {
            let #name { #(#field_names),* } = self;
        }),
        syn::Fields::Unnamed(_) => Some(quote::quote! {
            let #name ( #(#field_names),* ) = self;
        }),
        syn::Fields::Unit => unreachable!(),
    };

    quote::quote! {
        impl #impl_generics ::byte::TryWrite<::byte::ctx::Endian> for #name #ty_generics #where_clause {
            fn try_write(self, bytes: &mut [u8], ctx: ::byte::ctx::Endian) -> ::byte::Result<usize> {
                let mut offset = &mut 0;
                #extract_fields
                #(#field_writes)*
                Ok(*offset)
            }
        }
    }
}

fn impl_try_read(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        syn::Data::Struct(data) => impl_struct_read(&ast.ident, &data.fields, &ast.generics),
        _ => syn::Error::new(ast.span(), "TryRead can only be derived for structs")
            .to_compile_error(),
    }
}

fn impl_try_write(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &ast.data {
        syn::Data::Struct(data) => impl_struct_write(&ast.ident, &data.fields, &ast.generics),
        _ => syn::Error::new(ast.span(), "TryWrite can only be derived for structs")
            .to_compile_error(),
    }
}

#[proc_macro_derive(TryRead, attributes(byte))]
pub fn derive_try_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let gen = impl_try_read(&ast);
    gen.into()
}

#[proc_macro_derive(TryWrite, attributes(byte))]
pub fn derive_try_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let gen = impl_try_write(&ast);
    gen.into()
}

fn parse_field_attrs(attr: &syn::Attribute) -> Result<FieldAttributes, syn::Error> {
    let parser = Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated;
    let args = attr.meta.require_list()?.parse_args_with(parser)?;

    let mut attributes = FieldAttributes {
        read_ctx: None,
        write_ctx: None,
    };

    for arg in args {
        if arg.path.is_ident("read_ctx") {
            attributes.read_ctx = Some(arg.value);
        } else if arg.path.is_ident("write_ctx") {
            attributes.write_ctx = Some(arg.value);
        } else {
            return Err(syn::Error::new(arg.path.span(), "unknown attribute"));
        }
    }

    Ok(attributes)
}
