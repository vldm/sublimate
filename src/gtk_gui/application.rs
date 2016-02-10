//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

use core::Core;
use traits::Application;

use gtk;

use std::rc::Rc;

use gtk::traits::*;
use gtk::signal::Inhibit;

use core::syntax::{WHITE, BLACK};

use super::editor;

pub struct GtkApplication {
    window:gtk::Window,
    core:Rc<Core>,
}

impl Application for GtkApplication {
    type Error = String;
    
    fn new(core: Core) -> Result<GtkApplication, Self::Error> {
        if gtk::init().is_err() {
            
            return Err("Failed to initialize GTK.".to_owned());
        }

        let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
        window.set_title("Sublimate GTK test");
        window.set_window_position(gtk::WindowPosition::Center);
        window.set_default_size(800, 600);
        
        let font = "Monospace 10";
        let core = Rc::new(core);
        let drawing_area = editor::LineRenderer::new_drawing_area(400, 800,
                            1., font.to_owned(), core.clone(),
                            WHITE, BLACK);
        window.add(&drawing_area);



        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        
        Ok(GtkApplication{
            window:window,
            core:core
        })
    }
    
    fn run(&mut self) {
        self.window.show_all();
        gtk::main();
    }
}
