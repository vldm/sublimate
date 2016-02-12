use core::Core;
use traits::Application;

use gtk;

use std::rc::Rc;

use gtk::traits::*;
use gtk::signal::Inhibit;

use core::syntax::{WHITE, BLACK};

use super::editor;
use super::tree::ProjectTree;

pub struct GtkApplication {
    window: gtk::Window,
    editor: gtk::DrawingArea,
    sidebar: ProjectTree,
    core: Rc<Core>,
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
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2).unwrap();
        let (tree_view, sidebar) = ProjectTree::new(core.clone());
        hbox.pack_start(&tree_view, false, false, 0);
        let drawing_area = editor::LineRenderer::new_drawing_area(400, 800,
                            1., font.to_owned(), core.clone(),
                            WHITE, BLACK);

        hbox.pack_start(&drawing_area, true, true, 0);
        window.add(&hbox);

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        
        Ok(GtkApplication{
            window: window,
            core: core,
            sidebar: sidebar,
            editor: drawing_area
        })
    }
    
    fn run(&mut self) {
        self.window.show_all();
        gtk::main();
    }
}
