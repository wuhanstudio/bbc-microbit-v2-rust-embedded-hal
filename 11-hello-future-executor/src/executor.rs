use core::{
    future::Future,
    pin::Pin,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker { unsafe {dummy_raw_waker()} }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    unsafe fn dummy_raw_waker() -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

pub fn run_tasks(tasks: &mut [Pin<&mut dyn Future<Output = ()>>]) -> ! {
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        for task in tasks.iter_mut() {
            let _ = task.as_mut().poll(&mut cx);
        }
    }
}
