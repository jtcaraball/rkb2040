use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ExprPath, Ident, Index, LitInt, Result, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct MatrixDesc {
    columns: Vec<LitInt>,
    rows: Vec<LitInt>,
}

impl Parse for MatrixDesc {
    fn parse(input: ParseStream) -> Result<Self> {
        let desc;
        parenthesized!(desc in input);

        let col_tokens;
        let mut columns = Vec::<LitInt>::new();
        parenthesized!(col_tokens in desc);
        while !col_tokens.is_empty() {
            let val = col_tokens.parse::<LitInt>()?;
            columns.push(val);
            if col_tokens.peek(Token![,]) {
                col_tokens.parse::<Token![,]>()?;
            }
        }

        desc.parse::<Token![,]>()?;

        let row_tokens;
        let mut rows = Vec::<LitInt>::new();
        parenthesized!(row_tokens in desc);
        while !row_tokens.is_empty() {
            let val = row_tokens.parse::<LitInt>()?;
            rows.push(val);
            if row_tokens.peek(Token![,]) {
                row_tokens.parse::<Token![,]>()?;
            }
        }

        if desc.peek(Token![,]) {
            desc.parse::<Token![,]>()?;
        }
        Ok(Self { columns, rows })
    }
}

struct KeyDesc {
    x: LitInt,
    y: LitInt,
}

impl Parse for KeyDesc {
    fn parse(input: ParseStream) -> Result<Self> {
        let coords;
        parenthesized!(coords in input);
        let x = coords.parse::<LitInt>()?;
        coords.parse::<Token![,]>()?;
        let y = coords.parse::<LitInt>()?;
        Ok(Self { x, y })
    }
}

struct KeySeq(pub Vec<KeyDesc>);

impl Parse for KeySeq {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut seq = Vec::<KeyDesc>::new();
        while !input.is_empty() {
            seq.push(input.parse::<KeyDesc>()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self(seq))
    }
}

struct MatrixKeyDesc {
    state_type: Type,
    keys: KeySeq,
}

impl Parse for MatrixKeyDesc {
    fn parse(input: ParseStream) -> Result<Self> {
        let state_type = input.parse::<Type>()?;
        input.parse::<Token![,]>()?;
        let keys = input.parse::<KeySeq>()?;
        Ok(Self { state_type, keys })
    }
}

pub fn keys_to_states_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as MatrixKeyDesc);
    let state_type = desc.state_type;
    let key_count = desc.keys.0.len();
    let states = (0..key_count).map(|_| {
        quote! { #state_type }
    });
    quote!((
        #(
            #states
        ),*
    ))
    .into()
}

pub fn keys_to_states_init_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as MatrixKeyDesc);
    let state_type = desc.state_type;
    let key_count = desc.keys.0.len();
    let inits = (0..key_count).map(|_| {
        quote! { #state_type::new() }
    });
    quote!((
        #(
            #inits
        ),*
    ))
    .into()
}

struct MatrixKBDesc {
    kb: ExprPath,
    timer: Ident,
    matrix: MatrixDesc,
    keys: KeySeq,
}

impl Parse for MatrixKBDesc {
    fn parse(input: ParseStream) -> Result<Self> {
        let kb = input.parse::<ExprPath>()?;
        input.parse::<Token![,]>()?;
        let timer = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let matrix = input.parse::<MatrixDesc>()?;
        input.parse::<Token![,]>()?;
        let keys = input.parse::<KeySeq>()?;
        Ok(Self {
            kb,
            timer,
            matrix,
            keys,
        })
    }
}

macro_rules! wait_block {
    ($timer:expr, $t:tt) => {
        quote! {
            let start = $timer.get_counter();
            loop {
                let Some(dur) = $timer.get_counter().checked_duration_since(start) else {
                    continue;
                };
                if dur.to_nanos() >= $t {
                    break;
                }
            }
        }
    };
}

pub fn matrix_pin_check_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as MatrixKBDesc);
    let kb = desc.kb;
    let timer = desc.timer;
    let mut checks = Vec::<proc_macro2::TokenStream>::new();
    for (c_idx, _) in desc.matrix.columns.iter().enumerate() {
        let mut row_checks = Vec::<proc_macro2::TokenStream>::new();
        let mut col_settle_mask: u32 = 0;
        for (r_idx, row) in desc.matrix.rows.iter().enumerate() {
            let index = desc.keys.0.iter().position(|e| {
                e.x.base10_parse::<usize>().unwrap() == c_idx
                    && e.y.base10_parse::<usize>().unwrap() == r_idx
            });
            if let Some(index) = index {
                let index = Index::from(index);
                col_settle_mask |= 1 << row.base10_parse::<u8>().unwrap();
                let check = quote!({
                    let key_mask: u32 = 1 << #row;
                    let pressed = bank & key_mask == 0;
                    if #kb.keys.#index.pressed != pressed &&
                        #kb.keys.#index.debounce.update(#timer.get_counter(), pressed) {
                        #kb.keys.#index.pressed = pressed;
                        #kb.tx.send_byte(if pressed {#index + 0b1000_0000} else {#index});
                    }
                });
                row_checks.push(check);
            }
        }
        if !row_checks.is_empty() {
            let c_idx = Index::from(c_idx);
            let wait: proc_macro2::TokenStream = wait_block!(timer, 250);
            checks.push(quote!({
                let col = unsafe { #kb.matrix.0.#c_idx.take().unwrap_unchecked()};
                let col = col.into_push_pull_output_in_state(rp2040_hal::gpio::PinState::Low);
                #wait
                let bank = rp2040_hal::Sio::read_bank0();
                #kb.matrix.0.#c_idx = Some(col.into_pull_up_input());
                #(#row_checks)*
                while rp2040_hal::Sio::read_bank0() & #col_settle_mask != #col_settle_mask {}
            }));
        }
    }
    let checks_iter = checks.into_iter();
    quote!(
        #(
            {
                #checks_iter
            }
        )*
    )
    .into()
}

#[expect(clippy::cast_possible_truncation)]
pub fn matrix_pin_rx_check_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as MatrixKBDesc);
    let kb = desc.kb;
    let timer = desc.timer;
    let key_count = desc.keys.0.len() as u8;
    let mut checks = Vec::<proc_macro2::TokenStream>::new();
    for (c_idx, _) in desc.matrix.columns.iter().enumerate() {
        let mut row_checks = Vec::<proc_macro2::TokenStream>::new();
        let mut col_settle_mask: u32 = 0;
        for (r_idx, row) in desc.matrix.rows.iter().enumerate() {
            let index = desc.keys.0.iter().position(|e| {
                e.x.base10_parse::<usize>().unwrap() == c_idx
                    && e.y.base10_parse::<usize>().unwrap() == r_idx
            });
            if let Some(index) = index {
                let index = Index::from(index);
                col_settle_mask |= 1 << row.base10_parse::<u8>().unwrap();
                let check = quote!({
                    let key_mask: u32 = 1 << #row;
                    let pressed = bank & key_mask == 0;
                    if #kb.keys.#index.pressed != pressed &&
                        #kb.keys.#index.debounce.update(#timer.get_counter(), pressed) {
                        #kb.keys.#index.pressed = pressed;
                        #kb.sm.register_press(if pressed {#index + 0b1000_0000} else {#index});
                    }
                    if let Some(msg) = #kb.rx.receive_byte() {
                        #kb.sm.register_press(msg + #key_count);
                    }
                });
                row_checks.push(check);
            }
        }
        if !row_checks.is_empty() {
            let c_idx = Index::from(c_idx);
            let wait: proc_macro2::TokenStream = wait_block!(timer, 250);
            checks.push(quote!({
                let col = unsafe { #kb.matrix.0.#c_idx.take().unwrap_unchecked()};
                let col = col.into_push_pull_output_in_state(rp2040_hal::gpio::PinState::Low);
                #wait
                let bank = rp2040_hal::Sio::read_bank0();
                #kb.matrix.0.#c_idx = Some(col.into_pull_up_input());
                #(#row_checks)*
                while rp2040_hal::Sio::read_bank0() & #col_settle_mask != #col_settle_mask {}
            }));
        }
    }
    let checks_iter = checks.into_iter();
    quote!(
        #(
            {
                #checks_iter
            }
        )*
    )
    .into()
}
