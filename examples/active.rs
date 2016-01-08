extern crate libc;
extern crate strophe;

use std::{env, mem, ptr, str};
use std::ffi::{CStr, CString};

use libc::*;
use strophe::*;

extern "C" fn reply_handler(conn: *const xmpp_conn_t,
                            stanza: *const xmpp_stanza_t,
                            userdata: *const c_void) -> i32 {
    unsafe {
        let stanza_type = ptr_to_str(xmpp_stanza_get_type(stanza));
        if stanza_type != "error" {
            let query = xmpp_stanza_get_child_by_name(stanza, str_to_ptr("query"));

            println!("Active Session:");

            let mut item = xmpp_stanza_get_children(query);
            while !item.is_null() {
                let jid = get_stanza_attr(item, "jid").unwrap();
                println!("\t {}", jid);

                item = xmpp_stanza_get_next(item);
            }
        } else {
            println!("error: query failed");
        }

        xmpp_disconnect(conn);

        return 0;
    }
}

extern "C" fn conn_handler(conn: *const xmpp_conn_t,
                           status: xmpp_conn_event_t,
                           error: i32,
                           stream_error: *const xmpp_stream_error_t,
                           userdata: *const c_void) {
    unsafe {
        let ctx: *mut xmpp_ctx_t = mem::transmute(userdata);

        if status != XMPP_CONN_CONNECT {
            println!("disconnected");
            xmpp_stop(ctx);
            return;
        }

        // create iq stanza for the request
        let iq = xmpp_stanza_new(ctx);
        xmpp_stanza_set_name(iq, str_to_ptr("iq"));
        xmpp_stanza_set_type(iq, str_to_ptr("get"));
        xmpp_stanza_set_id(iq, str_to_ptr("active1"));
        xmpp_stanza_set_to(iq, str_to_ptr("xxxxxxxx.com"));

        // create query to request active resources on the server
        let query = xmpp_stanza_new(ctx);
        xmpp_stanza_set_name(query, str_to_ptr("query"));
        xmpp_stanza_set_ns(query, str_to_ptr(XMPP_NS_DISCO_ITEMS));
        xmpp_stanza_set_attribute(query,
                                  str_to_ptr("node"),
                                  str_to_ptr("sessions"));

        xmpp_stanza_add_child(iq, query);

        // release query stanza because it belongs to iq now
        xmpp_stanza_release(query);

        // register reply handler
        xmpp_id_handler_add(conn,
                            Some(reply_handler),
                            str_to_ptr("active1"),
                            mem::transmute(ctx));

        // send the stanza
        xmpp_send(conn, iq);

        // release the iq stanza
        xmpp_stanza_release(iq);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: ./active <jid> <pwd>");
        return;
    }

    let jid = args[1].clone();
    let pwd = args[2].clone();

    unsafe {
        // initialize library
        xmpp_initialize();

        // create a context
        let ctx = xmpp_ctx_new(ptr::null(), ptr::null());

        // create a connection
        let conn = xmpp_conn_new(ctx);

        // setup authentication information
        xmpp_conn_set_jid(conn, str_to_ptr(jid));
        xmpp_conn_set_pass(conn, str_to_ptr(pwd));

        // initialize the connection
        xmpp_connect_client(conn,
                            ptr::null(),
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

fn get_stanza_attr<'a>(item: *const xmpp_stanza_t,
                       attr: &str) -> Option<&'a str> {
    let res = unsafe { xmpp_stanza_get_attribute(item, str_to_ptr(attr)) };

    if !res.is_null() {
        Some(ptr_to_str(res))
    } else {
        None
    }
}

fn str_to_ptr<T: Into<Vec<u8>>>(input: T) -> *const i8 {
  CString::new(input).unwrap().as_bytes_with_nul().as_ptr() as *const i8
}

fn ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
  unsafe {
    str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap()
  }
}
