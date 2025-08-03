use proc_macro::TokenStream;

mod keyboard;

#[proc_macro]
pub fn direct_pin_check(input: TokenStream) -> TokenStream {
    keyboard::direct_pin_check_impl(input)
}

#[proc_macro]
pub fn direct_pin_rx_check(input: TokenStream) -> TokenStream {
    keyboard::direct_pin_rx_check_impl(input)
}
