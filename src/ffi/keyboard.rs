#![allow(non_camel_case_types, dead_code)]
use crate::mre_api;

pub const VM_KEY_EVENT_UP: i32 = 1;
pub const VM_KEY_EVENT_DOWN: i32 = 2;
pub const VM_KEY_EVENT_LONG_PRESS: i32 = 3;
pub const VM_KEY_EVENT_REPEAT: i32 = 4;

pub const VM_KEY_UP: i32 = -1;
pub const VM_KEY_DOWN: i32 = -2;
pub const VM_KEY_LEFT: i32 = -3;
pub const VM_KEY_RIGHT: i32 = -4;
pub const VM_KEY_OK: i32 = -5;
pub const VM_KEY_LEFT_SOFTKEY: i32 = -6;
pub const VM_KEY_RIGHT_SOFTKEY: i32 = -7;
pub const VM_KEY_CLEAR: i32 = -8;
pub const VM_KEY_BACK: i32 = -9;

pub const VM_KEY_NUM0: i32 = 48;
pub const VM_KEY_NUM1: i32 = 49;
pub const VM_KEY_NUM2: i32 = 50;
pub const VM_KEY_NUM3: i32 = 51;
pub const VM_KEY_NUM4: i32 = 52;
pub const VM_KEY_NUM5: i32 = 53;
pub const VM_KEY_NUM6: i32 = 54;
pub const VM_KEY_NUM7: i32 = 55;
pub const VM_KEY_NUM8: i32 = 56;
pub const VM_KEY_NUM9: i32 = 57;
pub const VM_KEY_VOL_UP: i32 = 58;
pub const VM_KEY_VOL_DOWN: i32 = 59;
pub const VM_KEY_POUND: i32 = 35;
pub const VM_KEY_STAR: i32 = 42;

pub const VM_KEY_A: i32 = 65;
pub const VM_KEY_B: i32 = 66;
pub const VM_KEY_C: i32 = 67;
pub const VM_KEY_D: i32 = 68;
pub const VM_KEY_E: i32 = 69;
pub const VM_KEY_F: i32 = 70;
pub const VM_KEY_G: i32 = 71;
pub const VM_KEY_H: i32 = 72;
pub const VM_KEY_I: i32 = 73;
pub const VM_KEY_J: i32 = 74;
pub const VM_KEY_K: i32 = 75;
pub const VM_KEY_L: i32 = 76;
pub const VM_KEY_M: i32 = 77;
pub const VM_KEY_N: i32 = 78;
pub const VM_KEY_O: i32 = 79;
pub const VM_KEY_P: i32 = 80;
pub const VM_KEY_Q: i32 = 81;
pub const VM_KEY_R: i32 = 82;
pub const VM_KEY_S: i32 = 83;
pub const VM_KEY_T: i32 = 84;
pub const VM_KEY_U: i32 = 85;
pub const VM_KEY_V: i32 = 86;
pub const VM_KEY_W: i32 = 87;
pub const VM_KEY_X: i32 = 88;
pub const VM_KEY_Y: i32 = 89;
pub const VM_KEY_Z: i32 = 90;
pub const VM_KEY_SPACE: i32 = 91;
pub const VM_KEY_TAB: i32 = 92;
pub const VM_KEY_DEL: i32 = 93;
pub const VM_KEY_ALT: i32 = 94;
pub const VM_KEY_CTRL: i32 = 95;
pub const VM_KEY_WIN: i32 = 96;
pub const VM_KEY_SHIFT: i32 = 97;
pub const VM_KEY_QUESTUIN: i32 = 98;
pub const VM_KEY_PERIOD: i32 = 99;
pub const VM_KEY_COMMA: i32 = 100;
pub const VM_KEY_EXCLAMATION: i32 = 101;
pub const VM_KEY_APOSTROPHE: i32 = 102;
pub const VM_KEY_AT: i32 = 103;
pub const VM_KEY_BACKSPACE: i32 = 104;
pub const VM_KEY_QWERTY_ENTER: i32 = 105;
pub const VM_KEY_FN: i32 = 106;
pub const VM_KEY_SYMBOL: i32 = 107;
pub const VM_KEY_NUM_LOCK: i32 = 108;
pub const VM_KEY_QWERTY_MENU: i32 = 109;

pub type vm_key_handler_t = extern "C" fn(event: i32, keycode: i32);

mre_api!(vm_reg_keyboard_callback(handler: vm_key_handler_t));

pub const VM_KEYPAD_1KEY_NUMBER: u8 = 0;
pub const VM_KEYPAD_2KEY_NUMBER: u8 = 1;
pub const VM_KEYPAD_3KEY_NUMBER: u8 = 2;
pub const VM_KEYPAD_1KEY_QWERTY: u8 = 3;
pub const VM_KEYPAD_2KEY_QWERTY: u8 = 4;
pub const VM_KEYPAD_3KEY_QWERTY: u8 = 5;

mre_api!(vm_kbd_set_mode(mode: u8) -> i32 = -1);