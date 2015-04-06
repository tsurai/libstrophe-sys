#![allow(non_camel_case_types)]

use libc::*;

pub type xmpp_handler = Option<extern "C" fn(*const xmpp_conn_t, *const xmpp_stanza_t, *const c_void)>;
pub type xmpp_conn_handler = Option<extern "C" fn(*const xmpp_conn_t, c_int, c_int, *const xmpp_stream_error_t, *const c_void)>;
pub type xmpp_timed_handler = Option<extern "C" fn(*const xmpp_conn_t, *const xmpp_stanza_t, *const c_void)>;
pub type xmpp_log_handler = Option<extern "C" fn(*const c_void, c_int, *const c_char, *const c_char, *const c_char)>;
pub type xmpp_open_handler = Option<extern "C" fn(*const xmpp_conn_t)>;

pub type hash_free_func = Option<extern "C" fn(*const xmpp_ctx_t, *mut c_void)>;

#[repr(C)]
pub struct xmpp_conn_t {
  _ref: c_uint,
  ctx: *mut xmpp_ctx_t,
  _type: c_int,

  state: c_int,
  timeout_stamp: uint64_t,
  error: c_int,
  stream_error: *mut xmpp_stream_error_t,
  sock: c_int,
  tls: *mut c_void,

  tls_support: c_int,
  tls_disabled: c_int,
  tls_failed: c_int,
  sasl_support: c_int,
  secured: c_int,
  
  bind_required: c_int,
  session_required: c_int,

  lang: *mut c_char,
  domain: *mut c_char,
  connectdomain: *mut c_char,
  connectport: *mut c_char,
  jid: *mut c_char,
  pass: *mut c_char,
  bound_jid: *mut c_char,
  stream_id: *mut c_char,

  blocking_seed: c_int,
  send_queue_max: c_int,
  send_queue_len: c_int,
  send_queue_head: *mut c_void,
  send_queue_tail: *mut c_void,

  reset_parser: c_int,
  parser: *mut c_void,
  connect_timeout: c_uint,
  open_handler: xmpp_open_handler,
  authenticated: c_int,

  conn_handler: xmpp_conn_handler,
  userdata: *mut c_void,

  timed_handlers: *mut c_void,
  id_handlers: *mut hash_t,
  handlers: *mut c_void
}

#[repr(C)]
pub struct xmpp_stanza_t {
  _ref: c_int,
  ctx: *mut xmpp_ctx_t,
  _type: c_int,
  prev: *mut xmpp_stanza_t,
  next: *mut xmpp_stanza_t,
  children: *mut xmpp_stanza_t,
  parent: *mut xmpp_stanza_t,
  data: *mut c_char,
  attributes: *mut hash_t
}

#[repr(C)]
pub struct xmpp_ctx_t {
  mem: *const xmpp_mem_t,
  log: *const xmpp_log_t,
  loop_status: c_int,
  connlist: *mut xmpp_connlist_t 
}

#[repr(C)]
pub struct xmpp_connlist_t {
  conn: *mut xmpp_conn_t, 
  next: *mut xmpp_connlist_t
}

#[repr(C)]
pub struct xmpp_mem_t {
  alloc: Option<extern "C" fn(size_t, *mut u8) -> *mut u8>,
  free: Option<extern "C" fn(*mut u8, *const u8)>,
  realloc: Option<extern "C" fn(*mut u8, size_t, *const u8) -> *mut u8>,
  userdata: *mut c_void
}

#[repr(C)]
pub struct xmpp_log_t {
  handler: xmpp_log_handler,
  userdata: *mut c_void
}

#[repr(C)]
pub struct xmpp_stream_error_t {
  _type: c_int, //xmpp_error_type_t
  text: *mut c_char,
  stanza: *mut xmpp_stanza_t
}

#[repr(C)]
pub struct hashentry_t {
  next: *mut hashentry_t,
  key: *mut c_char,
  values: *mut c_void
}

#[repr(C)]
pub struct hash_t {
  _ref: c_uint,
  ctx: *mut xmpp_ctx_t,
  free: hash_free_func,
  length: c_int,
  num_keys: c_int,
  entries: *mut *mut hashentry_t
}

#[link(name="strophe")]
extern {
  // connection management
  pub fn xmpp_conn_new(ctx: *const xmpp_ctx_t) -> *mut xmpp_conn_t;
  pub fn xmpp_conn_close(ctx: *const xmpp_ctx_t) -> *mut xmpp_conn_t;
  pub fn xmpp_conn_release(conn: *const xmpp_conn_t) -> c_int;
  pub fn xmpp_conn_get_jid(conn: *const xmpp_conn_t) -> *const c_char;
  pub fn xmpp_conn_get_bound_jid(conn: *const xmpp_conn_t) -> *const c_char;
  pub fn xmpp_conn_set_jid(conn: *const xmpp_conn_t, jid: *const c_char);
  pub fn xmpp_conn_get_pass(conn: *const xmpp_conn_t) -> *const c_char;
  pub fn xmpp_conn_set_pass(conn: *const xmpp_conn_t, pass: *const c_char);
  pub fn xmpp_conn_get_context(conn: *const xmpp_conn_t) -> *mut xmpp_ctx_t;
  pub fn xmpp_connect_client(conn: *const xmpp_conn_t, altdomain: *const c_char, altport: c_ushort, callback: xmpp_conn_handler, userdata: *const u8) -> c_int;
  pub fn xmpp_disconnect(conn: *const xmpp_conn_t);
  pub fn xmpp_send(conn: *const xmpp_conn_t, stanza: *const xmpp_stanza_t);

  // context objects
  pub fn xmpp_get_default_logger(level: c_int) -> *mut xmpp_log_t;
  pub fn xmpp_ctx_new(mem: *const xmpp_mem_t, log: *const xmpp_log_t) -> *mut xmpp_ctx_t;
  pub fn xmpp_ctx_free(ctx: *const xmpp_ctx_t);

  // initialization, shutdown and versioning
  pub fn xmpp_initialize();
  pub fn xmpp_shutdown();
  pub fn xmpp_version_check(major: c_int, minor: c_int) -> c_int;
 
  // event loop
  pub fn xmpp_run_once(ctx: *mut xmpp_ctx_t, timeout: c_ulong);
  pub fn xmpp_run(ctx: *mut xmpp_ctx_t);
  pub fn xmpp_stop(ctx: *mut xmpp_ctx_t);

  // stanza and timed event handlers
  pub fn xmpp_timed_handler_delete(conn: *const xmpp_conn_t, handler: xmpp_timed_handler);
  pub fn xmpp_id_handler_delete(conn: *const xmpp_conn_t, handler: xmpp_handler, id: *const c_char);
  pub fn xmpp_handler_delete(conn: *const xmpp_conn_t, handler: xmpp_handler);
  pub fn xmpp_timed_handler_add(conn: *const xmpp_conn_t, handler: xmpp_timed_handler, period: c_ulong, userdata: *const c_void);
  pub fn xmpp_id_handler_add(conn: *const xmpp_conn_t, handler: xmpp_handler, id: *const c_char, userdata: *const c_void);
  pub fn xmpp_handler_add(conn: *const xmpp_conn_t, handler: xmpp_handler, ns: *const c_char, name: *const c_char, _type: *const c_char, userdata: *const c_void);

  // stanza creation and manipulation
  pub fn xmpp_stanza_new(ctx: *mut xmpp_ctx_t) -> *mut xmpp_stanza_t;
  pub fn xmpp_stanza_clone(stanza: *const xmpp_stanza_t) -> *mut xmpp_stanza_t;
  pub fn xmpp_stanza_copy(stanza: *const xmpp_stanza_t) -> *mut xmpp_stanza_t;
  pub fn xmpp_stanza_release(stanza: *const xmpp_stanza_t) -> c_int;
  pub fn xmpp_stanza_is_text(stanza: *const xmpp_stanza_t) -> c_int;
  pub fn xmpp_stanza_is_tag(stanza: *const xmpp_stanza_t) -> c_int;
  pub fn xmpp_stanza_to_text(stanza: *mut xmpp_stanza_t, buf: *const *const c_char, sizelen: *const size_t) -> c_int;
  pub fn xmpp_stanza_set_name(stanza: *mut xmpp_stanza_t, name: *const c_char) -> c_int;
  pub fn xmpp_stanza_get_name(stanza: *const xmpp_stanza_t) -> *mut c_char;
  pub fn xmpp_stanza_get_attribute_count(stanza: *const xmpp_stanza_t) -> c_int;
  pub fn xmpp_stanza_get_attributes(stanza: *const xmpp_stanza_t, attr: *const *const c_char, attrlen: c_int) -> c_int;
  pub fn xmpp_stanza_set_attribute(stanza: *const xmpp_stanza_t, key: *const c_char, value: *const c_char) -> c_int;
  pub fn xmpp_stanza_set_ns(stanza: *const xmpp_stanza_t, ns: *const c_char) -> c_int;
  pub fn xmpp_stanza_add_child(stanza: *mut xmpp_stanza_t, child: *mut xmpp_stanza_t) -> c_int;
  pub fn xmpp_stanza_set_text(stanza: *mut xmpp_stanza_t, text: *const c_char) -> c_int;
  pub fn xmpp_stanza_set_text_with_size(stanza: *mut xmpp_stanza_t, text: *const c_char, size: size_t) -> c_int;
  pub fn xmpp_stanza_get_id(stanza: *const xmpp_stanza_t) -> *mut c_char;
  pub fn xmpp_stanza_get_ns(stanza: *const xmpp_stanza_t) -> *mut c_char;
  pub fn xmpp_stanza_get_type(stanza: *const xmpp_stanza_t) -> *mut c_char;
  pub fn xmpp_stanza_get_child_by_name(stanza: *const xmpp_stanza_t, name: *const c_char) -> *mut xmpp_stanza_t;
  pub fn xmpp_stanza_get_child_by_ns(stanza: *const xmpp_stanza_t, ns: *const c_char) -> *mut xmpp_stanza_t;
  pub fn xmpp_stanza_get_children(stanza: *const xmpp_stanza_t) -> *mut xmpp_stanza_t;
  pub fn xmpp_stanza_get_next(stanza: *const xmpp_stanza_t) -> *mut xmpp_stanza_t;
  pub fn xmpp_stanza_get_text(stanza: *const xmpp_stanza_t) -> *mut c_char;
  pub fn xmpp_stanza_get_text_ptr(stanza: *const xmpp_stanza_t) -> *mut c_char;
  pub fn xmpp_stanza_set_id(stanza: *const xmpp_stanza_t, id: *const c_char) -> c_int;
  pub fn xmpp_stanza_set_type(stanza: *const xmpp_stanza_t, _type: *const c_char) -> c_int;
  pub fn xmpp_stanza_get_attribute(stanza: *const xmpp_stanza_t, name: *const c_char) -> *mut c_char;
}
