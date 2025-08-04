use rkb2040_lib::{
    keymap::{Keybind, KeymapSM}, KC, LS, OS_LS
};

const KCOUNT: usize = 34;
const LCOUNT: usize = 1;

pub type Sweep2SM = KeymapSM<KCOUNT, LCOUNT>;

pub const KEYMAP: [[Keybind; KCOUNT]; LCOUNT] = [
    [
        // Left
        KC!(Q), KC!(W), KC!(E), KC!(R), KC!(T),
        KC!(A), KC!(S), KC!(D), KC!(F), KC!(G),
        KC!(Z), KC!(X), KC!(C), KC!(V), KC!(B),
        OS_LS!(), KC!(Space),
        // Right
        KC!(Y), KC!(U), KC!(I), KC!(O), KC!(P),
        KC!(H), KC!(J), KC!(K), KC!(L), KC!(Semicolon),
        KC!(N), KC!(M), KC!(Comma), KC!(Dot), KC!(ForwardSlash),
        LS!(), KC!(DeleteBackspace),
    ]
];
