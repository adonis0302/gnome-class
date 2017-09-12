#![feature(proc_macro)]

// FIXME: can we combine these two?
extern crate gnome_class_shims;
extern crate gobject_gen;
extern crate glib;

use gobject_gen::gobject_gen;
use gnome_class_shims::gobject_sys;
use gnome_class_shims::GInstance;
use std::cell::Cell;
use std::ffi::CStr;
use std::mem;
use std::slice;

gobject_gen! {
    class Signaler {
        struct SignalerPrivate {
            val: Cell<u32>
        }

        signal value_changed(&self);

        fn set_value(&self, v: u32) {
            let private = self.private();
            private.val.set(v);
            // private.emit_value_changed();
        }

        fn get_value(&self) -> u32 {
            let private = self.private();
            private.val.get()
        }
    }
}

#[test]
fn has_value_changed_signal() {
    let obj: Signaler = Signaler::new();
    let obj_type = Signaler::get_type();

    unsafe {
        let mut n_ids: libc::c_uint = mem::zeroed();

        let raw_signal_ids = gobject_sys::g_signal_list_ids(obj_type, &mut n_ids);
        assert_eq!(n_ids, 1);

        let n_ids = n_ids as usize;

        let signal_ids = slice::from_raw_parts(raw_signal_ids, n_ids);

        let mut query: gobject_sys::GSignalQuery = mem::zeroed();
        gobject_sys::g_signal_query(signal_ids[0], &mut query);

        assert_eq!(query.itype, obj_type);
        assert_eq!(query.signal_id, signal_ids[0]);

        let signal_name = CStr::from_ptr(query.signal_name);
        assert_eq!(signal_name.to_str().unwrap(), "value-changed");
    }
}