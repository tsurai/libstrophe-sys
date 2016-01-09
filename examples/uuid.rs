extern crate libc;
extern crate strophe;

use std::{ptr, str};
use std::ffi::CStr;

use libc::*;
use strophe::*;

/*
 * Example generating a new uuid
 *
 * Rust version of the original uuid.c example file
 * https://github.com/strophe/libstrophe/blob/master/examples/uuid.c
 */

fn main() {
    unsafe {
        let ctx = xmpp_ctx_new(ptr::null(), ptr::null());
        let uuid = xmpp_uuid_gen(ctx);

        if !uuid.is_null() {
            println!("{}", ptr_to_str(uuid));
            xmpp_free(ctx, uuid as *const c_void);
        } else {
            println!("couldn't allocate memory");
        }

        xmpp_ctx_free(ctx);
    }
}

fn ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
    unsafe {
        str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap()
    }
}
