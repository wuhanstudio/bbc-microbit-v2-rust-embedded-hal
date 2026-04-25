use core::{
    future::Future,
    pin::Pin,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

use core::sync::atomic::{AtomicBool, Ordering};

pub struct Task<'a> {
    pub future: Pin<&'a mut dyn Future<Output = ()>>,
    pub ready: AtomicBool,
}

fn task_waker(task: *const Task) -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }

    unsafe fn wake(data: *const ()) {
        let task = &*(data as *const Task);
        task.ready.store(true, Ordering::Release);
        cortex_m::asm::sev(); // wake CPU
    }

    unsafe fn wake_by_ref(data: *const ()) {
        wake(data);
    }

    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe { Waker::from_raw(RawWaker::new(task as *const (), &VTABLE)) }
}

pub fn run_tasks(tasks: &mut [Task]) -> ! {
    loop {
        let mut did_work = false;

        for task in tasks.iter_mut() {
            if task.ready.swap(false, Ordering::AcqRel) {
                let waker = task_waker(task as *const _);
                let mut cx = Context::from_waker(&waker);

                if task.future.as_mut().poll(&mut cx).is_ready() {
                    // optionally mark as finished
                }

                did_work = true;
            }
        }

        if !did_work {
            cortex_m::asm::wfe();
        }
    }
}
