#![allow(unused_variables)]

extern crate libc;
extern crate strophe;

use std::{env, mem, ptr};
use std::ffi::CString;
use std::str::FromStr;

use libc::*;
use strophe::*;

/*
 * Demonstration of a simple connection to a server
 * as an external component (See XEP-0114)
 *
 * Rust version of the original component.c example file
 * https://github.com/strophe/libstrophe/blob/master/examples/component.c
 */

extern "C" fn conn_handler(conn: *const xmpp_conn_t,
                           status: xmpp_conn_event_t,
                           error: i32,
                           stream_error: *const xmpp_stream_error_t,
                           userdata: *const c_void) {
    let ctx: *mut xmpp_ctx_t = unsafe { mem::transmute(userdata) };

    if status == XMPP_CONN_CONNECT {
        println!("connected");
        unsafe {xmpp_disconnect(conn) };
    } else {
        println!("disconnected");
        unsafe { xmpp_stop(ctx) };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 || args.len() > 5 {
        println!("usage: ./components <jid> <pwd> <host> [port]\n");
        return;
    }

    let jid = args[1].clone();
    let pwd = args[2].clone();
    let host = args[3].clone();

    let port: u16 = if args.len() == 5 {
        match u16::from_str(&args[4][..]) {
            Ok(p) => p,
            Err(err) => {
                println!("Can't parse port: {}", err);
                return;
            }
        }
    } else {
        0
    };

    unsafe {
        // initialize library
        xmpp_initialize();

        // create a context with logger set to debug level
        let log = xmpp_get_default_logger(XMPP_LEVEL_DEBUG);
        let ctx = xmpp_ctx_new(ptr::null(), log);

        // create a connection
        let conn = xmpp_conn_new(ctx);

        // setup authentication information
        xmpp_conn_set_jid(conn, str_to_ptr(jid));
        xmpp_conn_set_pass(conn, str_to_ptr(pwd));

        // initialize the connection
        xmpp_connect_client(conn,
                            str_to_ptr(host),
                            port,
                            Some(conn_handler),
                            ctx as *const c_void);

        // start the event loop
        xmpp_run(ctx);

        // release the connection and context
        xmpp_conn_release(conn);
        xmpp_ctx_free(ctx);

        // shutdown library
        xmpp_shutdown();
    }
}

fn str_to_ptr<T: Into<Vec<u8>>>(input: T) -> *const i8 {
    CString::new(input).unwrap().as_bytes_with_nul().as_ptr() as *const i8
}
