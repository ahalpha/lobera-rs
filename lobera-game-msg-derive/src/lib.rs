use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemFn, LitInt, parse_macro_input};

#[proc_macro_attribute]
pub fn func_reg(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;

    let expanded = quote! {
        #input

        inventory::submit! {
            crate::base::app::HandlerRegistration {
                register_fn: |app| {
                    app.register(#name);
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(MsgNo, attributes(msg_no))]
pub fn derive_msg_no(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut msg_no: Option<u16> = None;

    if let Some(attr) = input.attrs.iter().find(|a| a.path().is_ident("msg_no")) {
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                let value = meta.value()?.parse::<LitInt>()?;
                msg_no = Some(value.base10_parse().unwrap());
            }
            Ok(())
        });
    }

    let msg_no = msg_no.expect("MsgNo derive requires #[msg_no(id = ...)] attribute");

    let expanded = quote! {
        impl crate::GameMessage for #name {
            const MSG_NO: u16 = #msg_no;
        }

        inventory::submit! {
            crate::GameMessageRegistry {
                id: #msg_no,
                factory: || Box::new(#name::default()),
            }
        }
    };

    TokenStream::from(expanded)
}
