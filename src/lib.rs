#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate rocksdb;

#[macro_use(log)]
extern crate log;

extern crate chrono;
extern crate ksuid;
extern crate rand;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

// For base62
extern crate byteorder;
extern crate resize_slice;
extern crate time;

//: Library modules

pub mod constants;
pub mod externals;
pub mod helpers;
pub mod models;
pub mod processors;
pub mod resources;

pub mod db_conn;

// Expose DbConn for submodule use.
pub use db_conn::DbConn;
