use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Message, attributes(ivproto))]
pub fn derive_iv_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(f) => f.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let mut decode_arms = Vec::new();
    let mut encode_statements = Vec::new();
    let mut field_count_logic = Vec::new();

    for field in fields {
        let field_name = &field.ident;
        let mut tag: Option<u8> = None;
        let mut field_type = String::new();
        let mut is_repeated = false;
        let mut is_message = false;
        let mut is_map = false;
        let mut key_name: Option<String> = None;

        // #[ivproto(int | message, repeated | map, key = "id", tag = "0", repeated)]
        if let Some(attr) = field.attrs.iter().find(|a| a.path().is_ident("ivproto")) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("tag") {
                    let value = meta.value()?.parse::<syn::LitStr>()?;
                    tag = Some(value.value().parse().unwrap());
                } else if meta.path.is_ident("repeated") {
                    is_repeated = true;
                } else if meta.path.is_ident("message") {
                    is_message = true;
                } else if meta.path.is_ident("map") {
                    is_map = true;
                } else if meta.path.is_ident("key") {
                    let value = meta.value()?.parse::<syn::LitStr>()?;
                    key_name = Some(value.value());
                } else {
                    field_type = meta.path.get_ident().unwrap().to_string();
                }
                Ok(())
            })
            .unwrap();
        }

        if let Some(t) = tag {
            let read_logic = if is_map {
                let k_ident = syn::Ident::new(
                    key_name.as_ref().expect("Map needs key attribute"),
                    proc_macro2::Span::call_site(),
                );

                let mut v_ty_hint = quote! { _ };
                let mut current_ty = &field.ty;
                if let syn::Type::Path(tp) = current_ty {
                    if let Some(last_seg) = tp.path.segments.last() {
                        if last_seg.ident == "Option" {
                            if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                                if let Some(syn::GenericArgument::Type(inner_ty)) =
                                    args.args.first()
                                {
                                    current_ty = inner_ty;
                                }
                            }
                        }
                    }
                }
                if let syn::Type::Path(tp) = current_ty {
                    if let Some(last_seg) = tp.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                            if args.args.len() >= 2 {
                                if let syn::GenericArgument::Type(v_ty) = &args.args[1] {
                                    v_ty_hint = quote! { #v_ty };
                                }
                            }
                        }
                    }
                }

                quote! {
                    let map_len = reader.read_u8()?;
                    let mut map = msg.#field_name.take().unwrap_or_default();
                    for _ in 0..map_len {
                        let val: #v_ty_hint = lobera_ivproto::IvMessage::decode(reader)?;
                        let key = val.#k_ident.clone().ok_or_else(|| {
                            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Map key {} is None", stringify!(#k_ident)))
                        })?;
                        map.insert(key, val);
                    }
                    msg.#field_name = Some(map);
                }
            } else if is_repeated {
                let inner_read = get_read_call(&field_type, is_message);
                quote! {
                    let len = reader.read_u16()?;
                    let mut vec = msg.#field_name.take().unwrap_or_default();
                    for _ in 0..len {
                        vec.push(#inner_read);
                    }
                    msg.#field_name = Some(vec);
                }
            } else {
                let inner_read = get_read_call(&field_type, is_message);
                quote! { msg.#field_name = Some(#inner_read); }
            };

            decode_arms.push(quote! {
                #t => { #read_logic }
            });

            let write_logic = if is_map {
                quote! {
                    writer.write_u8(v.len() as u8)?;
                    for (_, val) in v.iter() {
                        val.encode(writer)?;
                    }
                }
            } else if is_repeated {
                let inner_write = get_write_call(&field_type, is_message);
                quote! {
                    writer.write_u16(v.len() as u16)?;
                    for val in v.iter() {
                        let v = val;
                        #inner_write
                    }
                }
            } else {
                let inner_write = get_write_call(&field_type, is_message);
                quote! {
                    let v = v;
                    #inner_write
                }
            };

            field_count_logic.push(quote! {
                if self.#field_name.is_some() {
                    count += 1;
                }
            });

            encode_statements.push(quote! {
                if let Some(ref v) = self.#field_name {
                    writer.write_u8(#t)?;
                    #write_logic
                }
            });
        }
    }

    let expanded = quote! {
        impl lobera_ivproto::IvMessage for #name {
            fn decode(reader: &mut lobera_ivproto::IvReader) -> std::io::Result<Self> {
                let mut msg = Self::default();
                let msg_size = reader.read_u8()?;
                for _ in 0..msg_size {
                    let field_index = reader.read_u8()?;
                    match field_index {
                        #(#decode_arms)*
                        _ => { }
                    }
                }
                Ok(msg)
            }

            fn encode(&self, writer: &mut lobera_ivproto::IvWriter) -> std::io::Result<()> {
                let mut count = 0u8;
                #(#field_count_logic)*
                writer.write_u8(count)?;
                #(#encode_statements)*
                Ok(())
            }
        }
    };
    TokenStream::from(expanded)
}

fn get_read_call(ty: &str, is_message: bool) -> proc_macro2::TokenStream {
    if is_message {
        quote! { lobera_ivproto::IvMessage::decode(reader)? }
    } else {
        match ty {
            "byte" => quote! { reader.read_u8()? },
            "short" => quote! { reader.read_i16()?.try_into().unwrap() },
            "ushort" => quote! { reader.read_u16()? },
            "int" => quote! { reader.read_i32()? },
            "uint" => quote! { reader.read_u32()? },
            "long" => quote! { reader.read_i64()? },
            "float" => quote! { reader.read_f32()? },
            "double" => quote! { reader.read_f64()? },
            "string" => quote! { reader.read_string()? },
            "bool" => quote! { reader.read_bool()? },
            "json" => quote! { reader.read_json()? },
            _ => quote! { Default::default() },
        }
    }
}

fn get_write_call(ty: &str, is_message: bool) -> proc_macro2::TokenStream {
    if is_message {
        quote! { v.encode(writer)? }
    } else {
        match ty {
            "byte" => quote! { writer.write_u8(*v)? },
            "short" => quote! { writer.write_i16(*v as i16)? },
            "ushort" => quote! { writer.write_u16(*v)? },
            "int" => quote! { writer.write_i32(*v)? },
            "uint" => quote! { writer.write_u32(*v)? },
            "long" => quote! { writer.write_i64(*v)? },
            "float" => quote! { writer.write_f32(*v)? },
            "double" => quote! { writer.write_f64(*v)? },
            "string" => quote! { writer.write_string(v)? },
            "bool" => quote! { writer.write_bool(*v)? },
            "json" => quote! { writer.write_json(v)? },
            _ => {
                quote! { return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unsupported type: {}", #ty))) }
            }
        }
    }
}
