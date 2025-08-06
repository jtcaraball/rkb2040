use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Result,
    parse::{Parse, ParseStream},
    {ExprPath, Ident, Index, LitInt, Token, parse_macro_input},
};

struct PinDesc {
    kb: ExprPath,
    timer: Ident,
    entries: Vec<LitInt>,
}

impl Parse for PinDesc {
    fn parse(input: ParseStream) -> Result<Self> {
        let kb = input.parse::<ExprPath>()?;
        input.parse::<Token![,]>()?;
        let timer = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        let mut entries = Vec::<LitInt>::new();
        assert!(!input.is_empty(), "Empty pin sequence");
        while !input.is_empty() {
            let val = input
                .parse::<syn::LitInt>()
                .expect("Elements of pin sequence must be ints");
            entries.push(val);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { kb, timer, entries })
    }
}

pub fn direct_pin_check_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as PinDesc);
    let kb = desc.kb;
    let timer = desc.timer;
    let pin_checks = desc.entries.into_iter().enumerate().map(|(i, pin_id)| {
        let index = Index::from(i);
        quote! {
            {
                let key_mask = 1 << #pin_id;
                let pressed = bank & key_mask == 0;
                if #kb.keys.#index.state.pressed != pressed &&
                    #kb.keys.#index.state.debounce.update(#timer.get_counter(), pressed) {
                    #kb.tx.send_byte(if pressed { #index + 0b1000_0000 } else { #index });
                    #kb.keys.#index.state.pressed = pressed;
                }
            }
        }
    });
    quote!({
        let bank = rp2040_hal::Sio::read_bank0();
        #(
            #pin_checks
        )*
    })
    .into()
}

#[expect(clippy::cast_possible_truncation)]
pub fn direct_pin_rx_check_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as PinDesc);
    let kb = desc.kb;
    let timer = desc.timer;
    let pin_count = desc.entries.len() as u8;
    let pin_checks = desc.entries.into_iter().enumerate().map(|(i, pin_id)| {
        let index = Index::from(i);
        quote! {
            {
                let key_mask = 1 << #pin_id;
                let pressed = bank & key_mask == 0;
                if #kb.keys.#index.state.pressed != pressed &&
                    #kb.keys.#index.state.debounce.update(#timer.get_counter(), pressed) {
                    #kb.keys.#index.state.pressed = pressed;
                    #kb.sm.register_press(if pressed {#index + 0b1000_0000} else {#index});
                }
                if let Some(msg) = #kb.rx.receive_byte() {
                    #kb.sm.register_press(msg + #pin_count)
                }
            }
        }
    });
    quote!({
        let bank = rp2040_hal::Sio::read_bank0();
        #(
            #pin_checks
        )*
    })
    .into()
}
