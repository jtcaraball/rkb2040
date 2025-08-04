use rkb2040_lib::{
    KC, LA, LC, LG, LS, MO, NA, OS_LA, OS_LC, OS_LG, OS_LS,
    keymap::{Keybind, KeymapSM},
};

const KCOUNT: usize = 34;
const LCOUNT: usize = 4;

pub type Sweep2SM = KeymapSM<KCOUNT, LCOUNT>;

pub const KEYMAP: [[Keybind; KCOUNT]; LCOUNT] = [
    // Base Layer
    [
        // Left
        KC!(Q), KC!(W), KC!(E), KC!(R), KC!(T),
        KC!(A), KC!(S), KC!(D), KC!(F), KC!(G),
        KC!(Z), KC!(X), KC!(C), KC!(V), KC!(B),
        MO!(2), KC!(Space),
        // Right
        KC!(Y), KC!(U), KC!(I), KC!(O), KC!(P),
        KC!(H), KC!(J), KC!(K), KC!(L), KC!(Semicolon),
        KC!(N), KC!(M), KC!(Comma), KC!(Dot), KC!(ForwardSlash),
        LS!(), MO!(1),
    ],
    // Layer 1
    [
        // Left
        NA!(), NA!(), NA!(), NA!(), KC!(Grave),
        LG!(), LS!(), LC!(), LA!(), KC!(Backslash),
        NA!(), NA!(), NA!(), NA!(), NA!(),
        MO!(2), KC!(Space),
        // Right
        KC!(Equal), KC!(Keyboard7), KC!(Keyboard8), KC!(Keyboard9), KC!(Keyboard0),
        KC!(Minus), KC!(Keyboard4), KC!(Keyboard5), KC!(Keyboard6), KC!(LeftBrace),
        KC!(Apostrophe), KC!(Keyboard1), KC!(Keyboard2), KC!(Keyboard3), KC!(RightBrace),
        NA!(), MO!(3),
    ],
    // Layer 2
    [
        // Left
        NA!(), NA!(), NA!(), LC!(R), NA!(),
        OS_LG!(), OS_LS!(), OS_LC!(), OS_LA!(), NA!(),
        LC!(Z), LC!(X), LC!(C), LC!(V), NA!(),
        NA!(), MO!(3),
        // Right
        NA!(), LC!(D), LC!(U), NA!(), NA!(),
        NA!(), KC!(Escape), KC!(Tab), KC!(DeleteBackspace), NA!(),
        KC!(LeftArrow), KC!(DownArrow), KC!(UpArrow), KC!(RightArrow), NA!(),
        KC!(ReturnEnter), NA!(),
    ],
    // Layer 3
    [
        // Left
        KC!(Mute), KC!(VolumeDown), KC!(VolumeUp), NA!(), KC!(Home),
        LG!(), LS!(), LC!(), LA!(), KC!(PageUp),
        NA!(), NA!(), NA!(), NA!(), KC!(PageDown),
        NA!(), NA!(),
        // Right
        KC!(DeleteForward), KC!(F7), KC!(F8), KC!(F9), KC!(F10),
        KC!(Insert), KC!(J), KC!(K), KC!(L), KC!(Semicolon),
        KC!(PrintScreen), KC!(F1), KC!(F2), KC!(F3), KC!(F12),
        NA!(), NA!(),
    ]
];
