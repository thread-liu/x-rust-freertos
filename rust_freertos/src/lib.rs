// Depress some warnings caused by our C bindings.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings; // This file is generated by bindgen and doesn't show up in the git repo.
mod port;
mod config;
mod projdefs;
mod trace;
mod ffi;
mod list;
mod queue;
mod task;
mod kernel;

use std::rc::Rc;
use std::cell::RefCell;

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

    #[test]
    fn test_trace() {
        traceQUEUE_CREATE!(1);
    }

    #[test]
    fn test_pdMS_TO_TICKS() {
        assert_eq!(1000, pdMS_TO_TICKS!(1000));
    }

    use list::*;
    #[test]
    fn test_list() {
        let item = ListItem::new(1);
        let list: Vec<Rc<RefCell<ListItem>>> = vec![];
        assert_eq!(true, list_is_empty!(list));
    }
}
