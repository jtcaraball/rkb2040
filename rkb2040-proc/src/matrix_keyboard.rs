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

struct MatrixSeq(pub Vec<MatrixDesc>);

impl Parse for MatrixSeq {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut seq = Vec::<MatrixDesc>::new();
        while !input.is_empty() {
            seq.push(input.parse::<MatrixDesc>()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self(seq))
    }
}

struct MatrixStateDesc {
    state_type: Type,
    matrices: MatrixSeq,
}

impl Parse for MatrixStateDesc {
    fn parse(input: ParseStream) -> Result<Self> {
        let state_type = input.parse::<Type>()?;
        input.parse::<Token![,]>()?;
        let matrices = input.parse::<MatrixSeq>()?;
        Ok(Self {
            state_type,
            matrices,
        })
    }
}

pub fn matrix_to_states_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as MatrixStateDesc);
    let state_type = desc.state_type;
    let key_count = desc
        .matrices
        .0
        .into_iter()
        .map(|m| m.columns.len() * m.rows.len())
        .sum();
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

pub fn matrix_to_states_init_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as MatrixStateDesc);
    let state_type = desc.state_type;
    let key_count = desc
        .matrices
        .0
        .into_iter()
        .map(|m| m.columns.len() * m.rows.len())
        .sum();
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

struct MatrixPinDesc {
    kb: ExprPath,
    timer: Ident,
    matrices: MatrixSeq,
}

impl Parse for MatrixPinDesc {
    fn parse(input: ParseStream) -> Result<Self> {
        let kb = input.parse::<ExprPath>()?;
        input.parse::<Token![,]>()?;
        let timer = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let matrices = input.parse::<MatrixSeq>()?;
        Ok(Self {
            kb,
            timer,
            matrices,
        })
    }
}

pub fn matrix_pin_check_impl(input: TokenStream) -> TokenStream {
    let desc = parse_macro_input!(input as MatrixPinDesc);
    let kb = desc.kb;
    let timer = desc.timer;
    let mut offset = 0;
    let mut checks = Vec::<proc_macro2::TokenStream>::new();
    for (m_idx, matrix) in desc.matrices.0.iter().enumerate() {
        let col_settle_mask: u32 = matrix
            .rows
            .iter()
            .map(|row| 1 << row.base10_parse::<u8>().unwrap())
            .reduce(|acc, e| acc | e)
            .unwrap();
        for (c_idx, _) in matrix.columns.iter().enumerate() {
            let row_checks = matrix.rows.iter().enumerate().map(|(r_idx, row)| {
                let index = Index::from((r_idx * matrix.columns.len()) + c_idx + offset);
                quote!({
                    let key_mask: u32 = 1 << #row;
                    let pressed = bank & key_mask == 0;
                    if #kb.keys.#index.pressed != pressed &&
                        #kb.keys.#index.debounce.update(#timer.get_counter(), pressed) {
                        #kb.keys.#index.pressed = pressed;
                        #kb.tx.send_byte(if pressed {#index + 0b1000_0000} else {#index});
                    }
                })
            });
            let m_idx = Index::from(m_idx);
            let c_idx = Index::from(c_idx);
            checks.push(quote!({
                let col = unsafe { #kb.matrices.#m_idx.0.#c_idx.take().unwrap_unchecked()};
                let col = col.into_push_pull_output_in_state(rp2040_hal::gpio::PinState::Low);
                let bank = rp2040_hal::Sio::read_bank0();
                #kb.matrices.#m_idx.0.#c_idx = Some(col.into_pull_up_input());
                #(#row_checks)*
                while rp2040_hal::Sio::read_bank0() & #col_settle_mask != #col_settle_mask {}
            }));
        }
        offset += matrix.columns.len() * matrix.rows.len();
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
    let desc = parse_macro_input!(input as MatrixPinDesc);
    let kb = desc.kb;
    let timer = desc.timer;
    let key_count: u8 = desc
        .matrices
        .0
        .iter()
        .map(|m| (m.columns.len() * m.rows.len()) as u8)
        .sum();
    let mut offset = 0;
    let mut checks = Vec::<proc_macro2::TokenStream>::new();
    for (m_idx, matrix) in desc.matrices.0.iter().enumerate() {
        let col_settle_mask: u32 = matrix
            .rows
            .iter()
            .map(|row| 1 << row.base10_parse::<u8>().unwrap())
            .reduce(|acc, e| acc | e)
            .unwrap();
        for (c_idx, _) in matrix.columns.iter().enumerate() {
            let row_checks = matrix.rows.iter().enumerate().map(|(r_idx, row)| {
                let index = Index::from((r_idx * matrix.columns.len()) + c_idx + offset);
                quote!({
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
                })
            });
            let m_idx = Index::from(m_idx);
            let c_idx = Index::from(c_idx);
            checks.push(quote!({
                let col = unsafe { #kb.matrices.#m_idx.0.#c_idx.take().unwrap_unchecked()};
                let col = col.into_push_pull_output_in_state(rp2040_hal::gpio::PinState::Low);
                let bank = rp2040_hal::Sio::read_bank0();
                #kb.matrices.#m_idx.0.#c_idx = Some(col.into_pull_up_input());
                #(#row_checks)*
                while rp2040_hal::Sio::read_bank0() & #col_settle_mask != #col_settle_mask {}
            }));
        }
        offset += matrix.columns.len() * matrix.rows.len();
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
