use proc_macro::TokenStream;

mod keyboards;

#[proc_macro]
pub fn direct_pin_check(input: TokenStream) -> TokenStream {
    keyboards::direct_pin_check_impl(input)
}

#[proc_macro]
pub fn direct_pin_rx_check(input: TokenStream) -> TokenStream {
    keyboards::direct_pin_rx_check_impl(input)
}
