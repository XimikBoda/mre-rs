use alloc::boxed::Box;
use crate::ffi::keyboard::*;
use crate::time::instant::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Up,
    Down,
    LongPress,
    Repeat,
    Unknown(i32),
}

impl From<i32> for KeyEvent {
    fn from(event: i32) -> Self {
        match event {
            VM_KEY_EVENT_UP => KeyEvent::Up,
            VM_KEY_EVENT_DOWN => KeyEvent::Down,
            VM_KEY_EVENT_LONG_PRESS => KeyEvent::LongPress,
            VM_KEY_EVENT_REPEAT => KeyEvent::Repeat,
            _ => KeyEvent::Unknown(event),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyCode(pub i32);

impl KeyCode {
    pub const UP: Self = Self(-1);
    pub const DOWN: Self = Self(-2);
    pub const LEFT: Self = Self(-3);
    pub const RIGHT: Self = Self(-4);
    pub const OK: Self = Self(-5);
    pub const LEFT_SOFTKEY: Self = Self(-6);
    pub const RIGHT_SOFTKEY: Self = Self(-7);
    pub const CLEAR: Self = Self(-8);
    pub const BACK: Self = Self(-9);

    pub const NUM0: Self = Self(48);
    pub const NUM1: Self = Self(49);
    pub const NUM2: Self = Self(50);
    pub const NUM3: Self = Self(51);
    pub const NUM4: Self = Self(52);
    pub const NUM5: Self = Self(53);
    pub const NUM6: Self = Self(54);
    pub const NUM7: Self = Self(55);
    pub const NUM8: Self = Self(56);
    pub const NUM9: Self = Self(57);
    pub const STAR: Self = Self(42);
    pub const POUND: Self = Self(35);

    pub const VOL_UP: Self = Self(58);
    pub const VOL_DOWN: Self = Self(59);

    pub const A: Self = Self(65);
    pub const B: Self = Self(66);
    pub const C: Self = Self(67);
    pub const D: Self = Self(68);
    pub const E: Self = Self(69);
    pub const F: Self = Self(70);
    pub const G: Self = Self(71);
    pub const H: Self = Self(72);
    pub const I: Self = Self(73);
    pub const J: Self = Self(74);
    pub const K: Self = Self(75);
    pub const L: Self = Self(76);
    pub const M: Self = Self(77);
    pub const N: Self = Self(78);
    pub const O: Self = Self(79);
    pub const P: Self = Self(80);
    pub const Q: Self = Self(81);
    pub const R: Self = Self(82);
    pub const S: Self = Self(83);
    pub const T: Self = Self(84);
    pub const U: Self = Self(85);
    pub const V: Self = Self(86);
    pub const W: Self = Self(87);
    pub const X: Self = Self(88);
    pub const Y: Self = Self(89);
    pub const Z: Self = Self(90);

    pub const SPACE: Self = Self(91);
    pub const TAB: Self = Self(92);
    pub const DEL: Self = Self(93);
    pub const ALT: Self = Self(94);
    pub const CTRL: Self = Self(95);
    pub const WIN: Self = Self(96);
    pub const SHIFT: Self = Self(97);
    pub const QUESTION: Self = Self(98);
    pub const PERIOD: Self = Self(99);
    pub const COMMA: Self = Self(100);
    pub const EXCLAMATION: Self = Self(101);
    pub const APOSTROPHE: Self = Self(102);
    pub const AT: Self = Self(103);
    pub const BACKSPACE: Self = Self(104);
    pub const QWERTY_ENTER: Self = Self(105);
    pub const FN: Self = Self(106);
    pub const SYMBOL: Self = Self(107);
    pub const NUM_LOCK: Self = Self(108);
    pub const QWERTY_MENU: Self = Self(109);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeypadMode {
    Number1Key = VM_KEYPAD_1KEY_NUMBER,
    Number2Key = VM_KEYPAD_2KEY_NUMBER,
    Number3Key = VM_KEYPAD_3KEY_NUMBER, //reserved
    Qwerty1Key = VM_KEYPAD_1KEY_QWERTY,
    Qwerty2Key = VM_KEYPAD_2KEY_QWERTY,
    Qwerty3Key = VM_KEYPAD_3KEY_QWERTY, //reserved
}

const FLAG_IS_PRESSED: u8    = 0b001;
const FLAG_JUST_PRESSED: u8  = 0b010;
const FLAG_JUST_RELEASED: u8 = 0b100;

#[derive(Clone, Copy)]
struct KeyState {
    flags: u8,
    time_value: u16,
}

impl KeyState {
    const fn new() -> Self {
        Self {
            flags: 0,
            time_value: 0,
        }
    }

    #[inline(always)]
    fn set_flag(&mut self, mask: u8, value: bool) {
        if value {
            self.flags |= mask;
        } else {
            self.flags &= !mask;
        }
    }

    #[inline(always)]
    fn get_flag(&self, mask: u8) -> bool {
        (self.flags & mask) != 0
    }
}

const MAX_KEYS: usize = 120;
static mut KEY_STATES: [KeyState; MAX_KEYS] = [KeyState::new(); MAX_KEYS];
static mut KEY_CALLBACK: Option<Box<dyn FnMut(KeyEvent, KeyCode)>> = None;
static mut CALLBACK_REGISTERED: bool = false;

#[inline(always)]
fn key_to_index(keycode: KeyCode) -> Option<usize> {
    let index = keycode.0 + 10;
    if index >= 0 && (index as usize) < MAX_KEYS {
        Some(index as usize)
    } else {
        None
    }
}

extern "C" fn global_key_router(event: i32, keycode: i32) {
    let now = Instant::now().ticks;
    let evt = KeyEvent::from(event);
    let key = KeyCode(keycode);

    unsafe {
        if let Some(idx) = key_to_index(KeyCode(keycode)) {
            let states_ptr = core::ptr::addr_of_mut!(KEY_STATES);
            let state = &mut (*states_ptr)[idx];

            match evt {
                KeyEvent::Down => {
                    if !state.get_flag(FLAG_IS_PRESSED) {
                        state.set_flag(FLAG_JUST_PRESSED, true);
                        state.time_value = now as u16; // Записуємо час старту
                    }
                    state.set_flag(FLAG_IS_PRESSED, true);
                }
                KeyEvent::Up => {
                    if state.get_flag(FLAG_IS_PRESSED) {
                        state.set_flag(FLAG_IS_PRESSED, false);
                        state.set_flag(FLAG_JUST_RELEASED, true);
                        state.time_value = (now as u16).wrapping_sub(state.time_value);
                    }
                }
                _ => {} 
            }
        }

        let cb_ptr = core::ptr::addr_of_mut!(KEY_CALLBACK);
        if let Some(cb) = (*cb_ptr).as_mut() {
            cb(evt, key);
        }
    }
}

fn ensure_registered() {
    unsafe {
        let reg_ptr = core::ptr::addr_of_mut!(CALLBACK_REGISTERED);
        if !*reg_ptr {
            vm_reg_keyboard_callback(global_key_router);
            *reg_ptr = true;
        }
    }
}

pub fn init(mode: KeypadMode) -> Result<(), i32> {
    unsafe {
        ensure_registered();
        
        let res = vm_kbd_set_mode(mode as u8);
        if res == 0 {
            Ok(())
        } else {
            Err(res)
        }
    }
}

pub fn set_handler<F>(handler: F)
where
    F: FnMut(KeyEvent, KeyCode) + 'static,
{
    unsafe {
        *core::ptr::addr_of_mut!(KEY_CALLBACK) = Some(Box::new(handler));
    }
}

#[inline]
pub fn is_pressed(keycode: KeyCode) -> bool {
    if let Some(idx) = key_to_index(keycode) {
        unsafe { (*core::ptr::addr_of!(KEY_STATES))[idx].get_flag(FLAG_IS_PRESSED) }
    } else {
        false
    }
}

#[inline]
pub fn just_pressed(keycode: KeyCode) -> bool {
    if let Some(idx) = key_to_index(keycode) {
        unsafe { (*core::ptr::addr_of!(KEY_STATES))[idx].get_flag(FLAG_JUST_PRESSED) }
    } else {
        false
    }
}

#[inline]
pub fn just_released(keycode: KeyCode) -> bool {
    if let Some(idx) = key_to_index(keycode) {
        unsafe { (*core::ptr::addr_of!(KEY_STATES))[idx].get_flag(FLAG_JUST_RELEASED) }
    } else {
        false
    }
}

pub fn hold_duration(keycode: KeyCode) -> u32 {
    if let Some(idx) = key_to_index(keycode) {
        unsafe {
            let state = &(*core::ptr::addr_of!(KEY_STATES))[idx];
            if state.get_flag(FLAG_IS_PRESSED) {
                (Instant::now().ticks as u16).wrapping_sub(state.time_value) as u32
            } else {
                state.time_value as u32
            }
        }
    } else {
        0
    }
}

pub fn update() {
    unsafe {
        let states_ptr = core::ptr::addr_of_mut!(KEY_STATES);
        for state in (*states_ptr).iter_mut() {
            state.set_flag(FLAG_JUST_PRESSED, false);
            state.set_flag(FLAG_JUST_RELEASED, false);
        }
    }
}