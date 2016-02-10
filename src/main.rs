#![allow(dead_code)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod core;

mod traits;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate glob;
extern crate onig;
extern crate unicode_width;

extern crate ncurses;

extern crate rustc_serialize;
extern crate weakjson;
extern crate plist;

extern crate clap;

extern crate gtk;
extern crate cairo;
extern crate pango;
extern crate glib;
extern crate pangocairo;

use clap::{App, Arg};

mod tui;
//\TODO quick hack, replace to local paths in modules
pub use tui::*;
use application::TuiApplication;

mod gtk_gui;
use gtk_gui::application::GtkApplication;

use traits::{Application};
use core::Core;

fn main() {

    let matches = App::new("sublimate")
                    .version(env!("CARGO_PKG_VERSION"))
                    .author("Ivan Ivaschenko <defuz@me.com>")
                    .about("ToDo")
                    .arg(Arg::with_name("packages")
                        .long("packages")
                        .value_name("PACKAGES PATH")
                        .help("Sets packages path")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("file")
                        .index(1)
                        .value_name("FILE PATH")
                        .help("Sets a path to viewing file")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("project")
                        .long("project")
                        .value_name("PROJECT PATH")
                        .help("Sets path to sublime project")
                        .takes_value(true)
                        .required(true))
                      .arg(Arg::with_name("tui")
                        .short("T")
                        .help("Set tui as frontend"))
                    .arg(Arg::with_name("gtk")
                        .short("G")
                        .conflicts_with("tui")
                        .help("Set gtk as frontend"))
                    .get_matches();

    let core = Core::load(
        matches.value_of("packages").unwrap(),
        matches.value_of("file").unwrap(),
        matches.value_of("project").unwrap());
        
    if matches.is_present("tui") {
        TuiApplication::new(core)
                         .map(|mut app| app.run())
                         .unwrap();
    }
    else if matches.is_present("gtk") {
        GtkApplication::new(core)
                         .map(|mut app| app.run())
                         .unwrap();
    }
    else{
        panic!("Set --tui or --gtk to run sublimate with specific user interface.");
    }

    // println!("{:?}", window);
 
    // core.package_repository.get_keymap("default/Default (OSX).sublime-keymap");
    // core.package_repository.get_keymap("default/Default
    // (Windows).sublime-keymap");
    // core.package_repository.get_keymap("default/Default (Linux).sublime-keymap");


    // info!("hello world!");

    // menu.render(Canvas { x1: 0, y1: 0, x2: 80, y2: 20});


    // Start ncurses.

    // mousemask(ALL_MOUSE_EVENTS as u64, None);

    // attron(COLOR_PAIR(150));

    // mvaddstr(10, 10, "hello world");
    // clear();
    // /* Print to the back buffer. */
    // // attron(COLOR_PAIR(0));
    // // printw("Базовый 0..15\n\n");
    // // for i in 0..15+1 {
    // //   attron(COLOR_PAIR(i));
    // //   printw("██");
    // // }

    // // printw("\n\nColor cube 16..231\n\n");
    // // for i in 16..231+1 {
    // //   attron(COLOR_PAIR(i));
    // //   printw("██");
    // // }

    // // printw("\n\nGrayscale 232..255\n\n");
    // // for i in 232..255+1 {
    // //   attron(COLOR_PAIR(i));
    // //   printw("██");
    // // }


    // Wait for a key press.
    // loop {
    //     let c = getch();
    //     println!("{:?}\r", c);
    //     if c == 10 {
    //         break;
    //     }
    // }


    
}
