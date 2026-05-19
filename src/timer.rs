use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::mre_callback;
use crate::ffi::timer::*;
use crate::time::instant::Instant;
use crate::app::ensure_sys_callback;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerType {
    Gui,
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerError {
    NoFreeTimers,
    ProcessError,
    ResourceError,
    MemoryError,
    UnknownError(i32),
}

impl From<i32> for TimerError {
    fn from(code: i32) -> Self {
        match code {
            VM_TIMER_MTK_TIMER_NO_FREE => TimerError::NoFreeTimers,
            VM_TIMER_ERROR_OF_PROCESS => TimerError::ProcessError,
            VM_TIMER_ERROR_OF_RES => TimerError::ResourceError,
            VM_TIMER_ERROR_OF_MEM => TimerError::MemoryError,
            _ => TimerError::UnknownError(code),
        }
    }
}


type IntervalHandler = Box<dyn FnMut()>;

struct IntervalSlot {
    tid: i32,
    timer_type: TimerType,
    handler: IntervalHandler,
}

static mut ACTIVE_INTERVALS: Option<Vec<IntervalSlot>> = None;

struct VirtualTimer {
    target_ticks: u32,
    handler: Box<dyn FnOnce()>,
}

static mut GUI_TIMEOUTS: Option<Vec<VirtualTimer>> = None;
static mut CURRENT_GUI_HW: Option<i32> = None;

static mut REGULAR_TIMEOUTS: Option<Vec<VirtualTimer>> = None;
static mut CURRENT_REGULAR_HW: Option<i32> = None;

static mut GUI_ACTIVE: bool = false;

#[inline]
fn time_left(target_ticks: u32, now_ticks: u32) -> u32 {
    let diff = target_ticks.wrapping_sub(now_ticks);
    if diff > 0x7FFFFFFF { 0 } else { diff }
}

mre_callback! {
    extern "C" fn gui_interval_router(tid: i32) { 
        process_interval(tid, TimerType::Gui); 
    }
}

mre_callback! {
    extern "C" fn regular_interval_router(tid: i32) { 
        process_interval(tid, TimerType::Regular); 
    }
}

fn process_interval(tid: i32, timer_type: TimerType) {
    unsafe {
        let intervals_ptr = core::ptr::addr_of_mut!(ACTIVE_INTERVALS);
        if let Some(intervals) = (*intervals_ptr).as_mut() {
            if let Some(slot) = intervals.iter_mut().find(|s| s.tid == tid && s.timer_type == timer_type) {
                (slot.handler)();
            }
        }
    }
}

mre_callback! {
    extern "C" fn gui_timeout_router(tid: i32) { 
        unsafe { 
            vm_delete_timer(tid); 
            *core::ptr::addr_of_mut!(CURRENT_GUI_HW) = None;
        }
        process_timeouts(TimerType::Gui); 
    }
}

mre_callback! {
    extern "C" fn regular_timeout_router(tid: i32) { 
        unsafe { 
            vm_delete_timer_ex(tid); 
            *core::ptr::addr_of_mut!(CURRENT_REGULAR_HW) = None;
        }
        process_timeouts(TimerType::Regular); 
    }
}

fn process_timeouts(timer_type: TimerType) {
    let mut to_execute = Vec::new();
    let now = Instant::now().ticks;

    unsafe {
        let queue_ptr = match timer_type {
            TimerType::Gui => core::ptr::addr_of_mut!(GUI_TIMEOUTS),
            TimerType::Regular => core::ptr::addr_of_mut!(REGULAR_TIMEOUTS),
        };

        if let Some(queue) = (*queue_ptr).as_mut() {
            while let Some(timer) = queue.last() {
                if time_left(timer.target_ticks, now) == 0 {
                    to_execute.push(queue.pop().unwrap());
                } else {
                    break;
                }
            }
        }
    }

    for timer in to_execute {
        (timer.handler)();
    }

    reschedule_virtual_hw_timer(timer_type);
}

fn reschedule_virtual_hw_timer(timer_type: TimerType) {
    unsafe {
        if timer_type == TimerType::Gui && !*core::ptr::addr_of_mut!(GUI_ACTIVE) {
            return;
        }

        let (queue_ptr, hw_ptr) = match timer_type {
            TimerType::Gui => (core::ptr::addr_of_mut!(GUI_TIMEOUTS), core::ptr::addr_of_mut!(CURRENT_GUI_HW)),
            TimerType::Regular => (core::ptr::addr_of_mut!(REGULAR_TIMEOUTS), core::ptr::addr_of_mut!(CURRENT_REGULAR_HW)),
        };

        if let Some(tid) = *hw_ptr {
            match timer_type {
                TimerType::Gui => vm_delete_timer(tid),
                TimerType::Regular => vm_delete_timer_ex(tid),
            };
            *hw_ptr = None;
        }

        if let Some(queue) = (*queue_ptr).as_ref() {
            if let Some(next_timer) = queue.last() {
                let delay = time_left(next_timer.target_ticks, Instant::now().ticks);
                // MRE не любить таймери на 0 мс, встановлюємо мінімум 1
                let safe_delay = if delay == 0 { 1 } else { delay };

                let new_tid = match timer_type {
                    TimerType::Gui => vm_create_timer(safe_delay, gui_timeout_router),
                    TimerType::Regular => vm_create_timer_ex(safe_delay, regular_timeout_router),
                };

                if new_tid >= 0 {
                    *hw_ptr = Some(new_tid);
                }
            }
        }
    }
}

pub fn suspend_gui_timers() {
    unsafe {
        *core::ptr::addr_of_mut!(GUI_ACTIVE) = false;
        
        let hw_ptr = core::ptr::addr_of_mut!(CURRENT_GUI_HW);
        if let Some(tid) = *hw_ptr {
            vm_delete_timer(tid);
            *hw_ptr = None;
        }
    }
}

pub fn resume_gui_timers() {
    unsafe {
        *core::ptr::addr_of_mut!(GUI_ACTIVE) = true;
    }
    process_timeouts(TimerType::Gui);
}

pub struct Timer {
    tid: i32,
    timer_type: TimerType,
}

impl Timer {
    pub fn interval<F>(ms: u32, timer_type: TimerType, callback: F) -> Result<Self, TimerError>
    where
        F: FnMut() + 'static,
    {
        if timer_type == TimerType::Gui {
            ensure_sys_callback();
        }

        let tid = unsafe {
            match timer_type {
                TimerType::Gui => vm_create_timer(ms, gui_interval_router),
                TimerType::Regular => vm_create_timer_ex(ms, regular_interval_router),
            }
        };

        if tid < 0 { return Err(TimerError::from(tid)); }

        unsafe {
            let ptr = core::ptr::addr_of_mut!(ACTIVE_INTERVALS);
            if (*ptr).is_none() {
                *ptr = Some(Vec::new());
            }
            (*ptr).as_mut().unwrap().push(IntervalSlot {
                tid,
                timer_type,
                handler: Box::new(callback),
            });
        }

        Ok(Self { tid, timer_type })
    }

    pub fn timeout<F>(delay_ms: u32, timer_type: TimerType, callback: F) 
    where
        F: FnOnce() + 'static,
    {
        if timer_type == TimerType::Gui {
            ensure_sys_callback();
        }

        let target_ticks = Instant::now().ticks.wrapping_add(delay_ms);

        unsafe {
            let queue_ptr = match timer_type {
                TimerType::Gui => core::ptr::addr_of_mut!(GUI_TIMEOUTS),
                TimerType::Regular => core::ptr::addr_of_mut!(REGULAR_TIMEOUTS),
            };

            if (*queue_ptr).is_none() {
                *queue_ptr = Some(Vec::new());
            }

            let queue = (*queue_ptr).as_mut().unwrap();
            
            queue.push(VirtualTimer {
                target_ticks,
                handler: Box::new(callback),
            });

            let now = Instant::now().ticks;
            queue.sort_by(|a, b| {
                let left_a = time_left(a.target_ticks, now);
                let left_b = time_left(b.target_ticks, now);
                left_b.cmp(&left_a) // Зворотне сортування
            });
        }

        reschedule_virtual_hw_timer(timer_type);
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            match self.timer_type {
                TimerType::Gui => vm_delete_timer(self.tid),
                TimerType::Regular => vm_delete_timer_ex(self.tid),
            };

            let ptr = core::ptr::addr_of_mut!(ACTIVE_INTERVALS);
            if let Some(intervals) = (*ptr).as_mut() {
                if let Some(index) = intervals.iter().position(|s| s.tid == self.tid && s.timer_type == self.timer_type) {
                    intervals.remove(index);
                }
            }
        }
    }
}