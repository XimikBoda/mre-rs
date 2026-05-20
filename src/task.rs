use core::future::Future;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use core::pin::Pin;
use core::cell::RefCell;
use alloc::rc::Rc;
use alloc::boxed::Box;

pub use crate::msg::post_task;

type LocalBoxFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

struct Task {
    future: RefCell<Option<LocalBoxFuture>>,
}

fn waker_clone(data: *const ()) -> RawWaker {
    let rc = unsafe { Rc::from_raw(data as *const Task) };
    let clone = rc.clone();
    core::mem::forget(rc);
    RawWaker::new(Rc::into_raw(clone) as *const (), &VTABLE)
}

fn waker_wake(data: *const ()) {
    let rc = unsafe { Rc::from_raw(data as *const Task) };
    let _ = post_task(move || poll_task(rc));
}

fn waker_wake_by_ref(data: *const ()) {
    let rc = unsafe { Rc::from_raw(data as *const Task) };
    let clone = rc.clone();
    core::mem::forget(rc);
    let _ = post_task(move || poll_task(clone));
}

fn waker_drop(data: *const ()) {
    unsafe { drop(Rc::from_raw(data as *const Task)) };
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);

fn poll_task(task: Rc<Task>) {
    let mut borrow = task.future.borrow_mut();
    if let Some(mut fut) = borrow.take() {
        let raw_waker = RawWaker::new(Rc::into_raw(task.clone()) as *const (), &VTABLE);
        let waker = unsafe { Waker::from_raw(raw_waker) };
        let mut cx = Context::from_waker(&waker);

        if fut.as_mut().poll(&mut cx).is_pending() {
            *borrow = Some(fut);
        }
    }
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    let task = Rc::new(Task {
        future: RefCell::new(Some(Box::pin(future))),
    });

    poll_task(task);
}

pub struct YieldNow {
    yielded: bool,
}

pub fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            return Poll::Ready(());
        }
        self.yielded = true;
        let waker = cx.waker().clone();
        let _ = post_task(move || waker.wake());
        Poll::Pending
    }
}