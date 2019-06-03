// Depress some warnings caused by our C bindings.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(fnbox)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simplelog;

mod bindings; // This file is generated by bindgen and doesn't show up in the git repo.
mod port;
mod config;
mod projdefs;
mod trace;
mod ffi;
mod list;
mod task_global;
pub mod task_control;
// mod task_api;
pub mod kernel;
mod queue;
mod queue_h;
mod task_queue;

#[cfg(test)]
mod tests {
    use super::*;

    /*
    // Note! This test SHOULD FAIL, showing something like this:
    // test tests::test_vPortYield ... error: process didn't exit successfully: `/rust_freertos/target/debug/deps/rust_freertos-f3432ee83a2dce9a` (signal: 11, SIGSEGV: invalid memory reference)
    #[test]
    fn test_portYIELD() {
        portYIELD!()
    }
    */

    /*
    // Note! This test SHOULD FAIL as well.
    // BUT on my Mac it just doesn't stop running. Weird.
    use port;
    #[test]
    fn test_port_start_scheduler() {
        port::port_start_scheduler();
    }
    */
    use std::sync::Arc;
    #[test]
    fn test_queue() {
        task_global::init();

        use queue::Queue;
        // 用new方法创建一个长度为10的queue。
        let q: Queue<u32> = Queue::new(10);

        // 两个任务共享所有权，所以需Arc包装。
        let shared_queue = Arc::new(&q);

        // 发送数据的任务代码。
        let sender = move || {
            let v = Arc::clone(&shared_queue);
            for i in 1..11 {
                // send方法的参数包括要发送的数据和ticks_to_wait
                v.send(i, pdMS_TO_TICKS!(50)).unwrap();
            }
        };

        // 接收数据的任务代码。
        let receiver = move || {
            let v = Arc::clone(&shared_queue);
            let sum = 0;
            loop {
                // receive方法的参数只有ticks_to_wait
                if let Some(x) = v.receive(pdMS_TO_TICKS!(300)) {
                    sum += x;
                } else {
                    // 若等待300ms仍未收到数据，则认为发送结束。
                    assert_eq!(sum, 55);
                    kernel::task_end_scheduler();
                }
            }
        };

        // 创建这两个任务。
        let sender_task = task_control::TCB::new()
                            .name("Sender")
                            .priority(3)
                            .initialise(sender);

        let receiver_task = task_control::TCB::new()
                            .name("Receiver")
                            .priority(3)
                            .initialise(receiver);

        kernel::task_start_scheduler();
    }
}
