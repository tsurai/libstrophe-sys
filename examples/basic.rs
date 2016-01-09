#![allow(unused_variables)]

extern crate libc;
extern crate strophe;

use std::{env, mem, ptr};
use std::ffi::CString;

use libc::*;
use strophe::*;

extern "C" fn conn_handler(conn: *const xmpp_conn_t,
                           status: xmpp_conn_event_t,
                           error: i32,
                           stream_error: *const xmpp_stream_error_t,
                           userdata: *const c_void) {
    let ctx: *mut xmpp_ctx_t = unsafe { mem::transmute(userdata) };

    if status == XMPP_CONN_CONNECT {
        println!("connected");
        let secured = unsafe { xmpp_conn_is_secured(conn) };
        println!("connection is {}",
                 if secured == 1 { "secured" } else { "NOT secured" });
    } else {
        println!("disconnected");
        unsafe { xmpp_stop(ctx) };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut flags: i64 = 0;
    let mut num_flags = 0;

    for arg in args.iter().skip(1).by_ref() {
        match &arg[..] {
            "--disable-tls" => {
                flags |= XMPP_CONN_FLAG_DISABLE_TLS;
            },
            "--mandatory-tls" => {
                flags |= XMPP_CONN_FLAG_MANDATORY_TLS;
            },
            "--legacy-ssl" => {
                flags |= XMPP_CONN_FLAG_LEGACY_SSL;
            },
            _ => {
                break;
            }
        }
        num_flags += 1;
    }

    if (args.len() - num_flags) < 3 {
        println!("usage: ./basic [options] <jid> <pwd> [<host>]\n
Options:\n
  --disable-tls     Disable TLS\n
  --mandatory-tls   Deny plaintext connection\n
  --legacy-ssl      Use old style SSL\n\n
Note: --disable-tls conflicts with --mandatory-tls or --legacy-ssl");
        return;
    }

    let jid = args[num_flags + 1].clone();
    let pwd = args[num_flags + 2].clone();

    let host = if num_flags + 3 < args.len() {
        Some(args[num_flags + 3].clone())
    } else {
        None
    };

    unsafe {
        // initialize library
        xmpp_initialize();

        // create a context
        let ctx = xmpp_ctx_new(ptr::null(), ptr::null());

        // create a connection
        let conn = xmpp_conn_new(ctx);

        // configure connection properties (optional)
        xmpp_conn_set_flags(conn, flags);

        // setup authentication information
        xmpp_conn_set_jid(conn, str_to_ptr(jid));
        xmpp_conn_set_pass(conn, str_to_ptr(pwd));

        // initialize the connection
        xmpp_connect_client(conn,
                            host.map_or(ptr::null(), |x| str_to_ptr(x)),
                            0,
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
