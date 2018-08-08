#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate habits;

#[macro_use]
extern crate log;

extern crate rocket;

extern crate rocket_contrib;
extern crate rocksdb;

mod app;

//use std::fs::File;
//use std::io::Read;
//use std::path::Path;

fn main() {
    //    let confpath = Path::new("config/habits.yml");
    //    let mut confbuf = String::new();
    //
    //    File::open(&confpath)
    //        .expect(&format!("Read config from {}", confpath))
    //        .read_to_string(&mut confbuf)
    //        .expect(&format!("Read config into string"));

    match app::init_app() {
        Ok(rocket) => error!("{}", rocket.launch()),
        Err(error) => error!("{}", error),
    };
}
