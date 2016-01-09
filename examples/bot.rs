#![allow(unused_variables)]

extern crate libc;
extern crate strophe;

use std::{env, mem, ptr, str};
use std::ffi::{CStr, CString};

use libc::*;
use strophe::*;

extern "C" fn version_handler(conn: *const xmpp_conn_t,
                              stanza: *const xmpp_stanza_t,
                              userdata: *const c_void) -> i32 {
    unsafe {
        let ctx: *mut xmpp_ctx_t = mem::transmute(userdata);

        println!("Received version request from {}",
                 ptr_to_str(xmpp_stanza_get_from(stanza)));

        let reply = xmpp_stanza_reply(stanza);
        xmpp_stanza_set_type(reply, str_to_ptr("result"));

        let query = xmpp_stanza_new(ctx);
        xmpp_stanza_set_name(query, str_to_ptr("query"));

        let ns = xmpp_stanza_get_ns(xmpp_stanza_get_children(stanza));
        if !ns.is_null() {
            xmpp_stanza_set_ns(query, ns);
        }

        let name = xmpp_stanza_new(ctx);
        xmpp_stanza_set_name(name, str_to_ptr("name"));
        xmpp_stanza_add_child(query, name);

        let mut text = xmpp_stanza_new(ctx);
        xmpp_stanza_set_text(text, str_to_ptr("libstrophe example bot"));
        xmpp_stanza_add_child(name, text);

        let version = xmpp_stanza_new(ctx);
        xmpp_stanza_set_name(version, str_to_ptr("version"));
        xmpp_stanza_add_child(query, version);

        text = xmpp_stanza_new(ctx);
        xmpp_stanza_set_text(text, str_to_ptr("1.0"));
        xmpp_stanza_add_child(version, text);

        xmpp_stanza_add_child(reply, query);

        xmpp_send(conn, reply);
        xmpp_stanza_release(reply);

        return 1;
    }
}

// handler for connection events
extern "C" fn message_handler(conn: *const xmpp_conn_t,
                              stanza: *const xmpp_stanza_t,
                              userdata: *const c_void) -> i32 {
    unsafe {
        let ctx: *mut xmpp_ctx_t = mem::transmute(userdata);

        if xmpp_stanza_get_child_by_name(stanza, str_to_ptr("body")).is_null() {
            return 1;
        }

        if !xmpp_stanza_get_type(stanza).is_null()
            && ptr_to_str(xmpp_stanza_get_type(stanza)) == "error" {
            return 1;
        }

        let intext = xmpp_stanza_get_text(
            xmpp_stanza_get_child_by_name(stanza, str_to_ptr("body")));

        println!("Incoming message from {}: {}", ptr_to_str(xmpp_stanza_get_from(stanza)), ptr_to_str(intext));

        let reply = xmpp_stanza_reply(stanza);
        if xmpp_stanza_get_type(reply).is_null() {
            xmpp_stanza_set_type(reply, str_to_ptr("char"));
        }

        let body = xmpp_stanza_new(ctx);
        xmpp_stanza_set_name(body, str_to_ptr("body"));

        let reply_text = format!("{} to you too!", ptr_to_str(intext));
        xmpp_free(ctx, intext as *const c_void);

        let text = xmpp_stanza_new(ctx);
        xmpp_stanza_set_text(text, str_to_ptr(reply_text));
        xmpp_stanza_add_child(body, text);
        xmpp_stanza_add_child(reply, body);
        xmpp_stanza_release(body);
        xmpp_stanza_release(text);

        xmpp_send(conn, reply);
        xmpp_stanza_release(reply);

        return 1;
    }
}

extern "C" fn conn_handler(conn: *const xmpp_conn_t,
                           status: xmpp_conn_event_t,
                           error: i32,
                           stream_error: *const xmpp_stream_error_t,
                           userdata: *const c_void) {
    unsafe {
        let ctx: *mut xmpp_ctx_t = mem::transmute(userdata);

        if status == XMPP_CONN_CONNECT {
            println!("connected");

            xmpp_handler_add(conn,
                             Some(version_handler),
                             str_to_ptr("jabber:iq:version"),
                             str_to_ptr("iq"),
                             ptr::null(),
                             ctx as *const c_void);
            xmpp_handler_add(conn,
                             Some(message_handler),
                             ptr::null(),
                             str_to_ptr("message"),
                             ptr::null(),
                             ctx as *const c_void);

            let pres = xmpp_stanza_new(ctx);
            xmpp_stanza_set_name(pres, str_to_ptr("presence"));
            xmpp_send(conn, pres);
            xmpp_stanza_release(pres);
        } else {
            println!("disconnected");
            xmpp_stop(ctx);
        }
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

fn str_to_ptr<T: Into<Vec<u8>>>(input: T) -> *const i8 {
    CString::new(input).unwrap().as_bytes_with_nul().as_ptr() as *const i8
}

fn ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
    unsafe {
        str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap()
    }
}
