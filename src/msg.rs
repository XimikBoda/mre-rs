use crate::ffi::msg::*;
use crate::ffi::pmng::VM_P_HANDLE;
use crate::process::Process;
use alloc::collections::VecDeque;
use alloc::boxed::Box;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgError {
    TargetIsSelf,
    InvalidMsgId,
    QueueFull,
}

const INTERNAL_TASK_MSG_ID: u32 = VM_MESSAGE_ID_BASE + 0x7777;

type UserMsgHandler = Box<dyn FnMut(Option<Process>, u32, i32, i32) -> i32>;

static mut APP_MSG_HANDLER: Option<UserMsgHandler> = None;
static mut MSG_CALLBACK_REGISTERED: bool = false;

static mut LOCAL_TASK_QUEUE: Option<VecDeque<Box<dyn FnOnce()>>> = None;
static mut PUMP_SCHEDULED: bool = false;
static mut IS_DISPATCHING: bool = false;

fn ensure_msg_callback() {
    unsafe {
        if !MSG_CALLBACK_REGISTERED {
            vm_reg_msg_proc(global_msg_router);
            MSG_CALLBACK_REGISTERED = true;
        }
    }
}

extern "C" fn global_msg_router(sender_handle: VM_P_HANDLE, msg_id: u32, wparam: i32, lparam: i32) -> i32 {
    let my_handle = Process::current().map(|p| p.handle()).unwrap_or(-1);

    if msg_id == INTERNAL_TASK_MSG_ID && sender_handle == my_handle && my_handle != 0 {
        unsafe { 
            *core::ptr::addr_of_mut!(PUMP_SCHEDULED) = false; 
            
            let queue_ptr = core::ptr::addr_of_mut!(LOCAL_TASK_QUEUE);
            
            if let Some(mut tasks) = (*queue_ptr).take() {
                while let Some(task) = tasks.pop_front() {
                    task();
                }
            }
        }
        return 0;
    }

    unsafe {
        let handler_ptr = core::ptr::addr_of_mut!(APP_MSG_HANDLER);

        if let Some(handler) = (*handler_ptr).as_mut() {
            let sender = Process::from_handle(sender_handle);
            return handler(sender, msg_id, wparam, lparam);
        }
    }

    0
}

pub fn set_msg_handler<F>(handler: F)
where
    F: FnMut(Option<Process>, u32, i32, i32) -> i32 + 'static,
{
    ensure_msg_callback();
    unsafe {
        APP_MSG_HANDLER = Some(Box::new(handler));
    }
}

pub fn post_task<F>(task: F) -> Result<(), ()>
where
    F: FnOnce() + 'static,
{
    ensure_msg_callback();

    unsafe {
        let queue_ptr = core::ptr::addr_of_mut!(LOCAL_TASK_QUEUE);
        
        if (*queue_ptr).is_none() {
            *queue_ptr = Some(VecDeque::new());
        }
        
        if let Some(queue) = (*queue_ptr).as_mut() {
            queue.push_back(Box::new(task));
        }

        let pump_ptr = core::ptr::addr_of_mut!(PUMP_SCHEDULED);
        
        if !*pump_ptr {
            let my_handle = Process::current().map(|p| p.handle()).unwrap_or(0);
            
            if my_handle != 0 {
                let res = vm_post_msg(my_handle, INTERNAL_TASK_MSG_ID, 0, 0);
                
                if res == 1 {
                    *pump_ptr = true;
                } else {
                    let disp_ptr = core::ptr::addr_of_mut!(IS_DISPATCHING);
                    
                    if !*disp_ptr {
                        *disp_ptr = true;
                        vm_appcomm_dispatch_msg(); 
                        *disp_ptr = false;

                        vm_post_msg(my_handle, INTERNAL_TASK_MSG_ID, 0, 0);
                    }
                }
            }
        }
    }
    
    Ok(())
}