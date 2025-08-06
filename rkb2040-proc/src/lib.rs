use proc_macro::TokenStream;
mod direct_keyboard;
mod matrix_keyboard;

#[proc_macro]
pub fn direct_pin_check(input: TokenStream) -> TokenStream {
    direct_keyboard::direct_pin_check_impl(input)
}

#[proc_macro]
pub fn direct_pin_rx_check(input: TokenStream) -> TokenStream {
    direct_keyboard::direct_pin_rx_check_impl(input)
}

#[proc_macro]
pub fn keys_to_states(input: TokenStream) -> TokenStream {
    matrix_keyboard::keys_to_states_impl(input)
}

#[proc_macro]
pub fn keys_to_states_init(input: TokenStream) -> TokenStream {
    matrix_keyboard::keys_to_states_init_impl(input)
}

#[proc_macro]
pub fn matrix_pin_check(input: TokenStream) -> TokenStream {
    matrix_keyboard::matrix_pin_check_impl(input)
}

#[proc_macro]
pub fn matrix_pin_rx_check(input: TokenStream) -> TokenStream {
    matrix_keyboard::matrix_pin_rx_check_impl(input)
}
