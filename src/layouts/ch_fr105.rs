//! Swiss (french) keyboard support

use crate::{
    DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers, PhysicalKeyboard,
};

/// A standard Swiss (french) 102-key (or 105-key including Windows keys) keyboard.
pub struct ChFr105Key;

impl KeyboardLayout for ChFr105Key {
    #[rustfmt::skip]
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8      => modifiers.handle_symbol2('§', '°'),
            KeyCode::Key1      => modifiers.handle_symbol3('1', '+', '|'),
            KeyCode::Key2      => modifiers.handle_symbol3('2', '"', '@'),
            KeyCode::Key3      => modifiers.handle_symbol3('3', '*', '#'),
            KeyCode::Key4      => modifiers.handle_symbol2('4', 'ç'),
            KeyCode::Key5      => modifiers.handle_symbol2('5', '%'),
            KeyCode::Key6      => modifiers.handle_symbol3('6', '&', '¬'),
            KeyCode::Key7      => modifiers.handle_symbol3('7', '/', '|'),
            KeyCode::Key8      => modifiers.handle_symbol3('8', '(', '¢'),
            KeyCode::Key9      => modifiers.handle_symbol2('9', ')'),
            KeyCode::Key0      => modifiers.handle_symbol2('0', '='),
            KeyCode::OemMinus  => modifiers.handle_symbol3('\'', '?', '´'),
            KeyCode::OemPlus   => modifiers.handle_symbol3('^', '`', '~'),
            // ========= Row 3 (QWERTY) =========
            KeyCode::Q         => modifiers.handle_ascii_2('Q', handle_ctrl),
            KeyCode::E         => modifiers.handle_ascii_3('E', '€', handle_ctrl),
            KeyCode::Y         => modifiers.handle_ascii_2('Z', handle_ctrl),
            KeyCode::Oem4      => modifiers.handle_symbol3('è', 'ü', '['),
            KeyCode::Oem6      => modifiers.handle_symbol3('¨', '!', ']'),
            // ========= Row 4 (ASDFG) =========
            KeyCode::Oem1      => modifiers.handle_symbol2('é', 'ö'),
            KeyCode::Oem3      => modifiers.handle_symbol3('à', 'ä', '{'),
            KeyCode::Oem7      => modifiers.handle_symbol3('$', '£', '}'),
            // ========= Row 5 (ZXCVB) =========
            KeyCode::Oem5      => modifiers.handle_symbol3('<', '>', '\\'),
            KeyCode::Z         => modifiers.handle_ascii_2('Y', handle_ctrl),
            KeyCode::M         => modifiers.handle_ascii_3('M', 'µ', handle_ctrl),
            KeyCode::OemComma  => modifiers.handle_symbol2(',', ';'),
            KeyCode::OemPeriod => modifiers.handle_symbol2('.', ':'),
            KeyCode::Oem2      => modifiers.handle_symbol2('-', '_'),
            // ========= Fallback =========
            e => super::Us104Key.map_keycode(e, modifiers, handle_ctrl),
        }
    }

    fn get_physical(&self) -> PhysicalKeyboard {
        PhysicalKeyboard::Iso
    }
}
