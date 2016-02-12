use gtk::{TreeView, TreeStore, TreeViewColumn, TreeIter, CellRendererText} ;
use glib::types::Type;
use glib::Value;
use gtk::traits::widget::WidgetTrait;
use core::Core;
use std::rc::Rc;

use core::workspace::{Folder};

use std::mem;
use gtk_ffi;

pub struct ProjectTree {
    core: Rc<Core>,
    store: TreeStore,
    view: TreeView,
}


fn redraw_tree_view( view: &TreeView) {
    unsafe {
        gtk_ffi::gtk_widget_queue_draw(mem::transmute(view.pointer));
    }
}

impl ProjectTree {
    pub fn new(core: Rc<Core>) -> (TreeView, ProjectTree) {

        let tree_store = TreeStore::new(&[Type::String]).unwrap();
        let tree_view = TreeView::new_with_model(&tree_store.get_model()
                                                            .unwrap()).unwrap();

        let mut tree = ProjectTree {
            core: core,
            store: tree_store,
            view: tree_view.clone()
        };
        let column = TreeViewColumn::new().unwrap();
        //column.set_title("test");
        
        let render = CellRendererText::new().unwrap();
        column.pack_start(&render, true);
        column.add_attribute(&render, "text",0);
        tree_view.append_column(&column);
        
        tree.init_data();
        let (_, height) =  tree_view.get_size_request();
        tree_view.set_size_request(60,height);
        (tree_view, tree)
    }
    fn append_name(store :&TreeStore, iter: Option<&TreeIter>, name:&str)
             -> TreeIter {
        let iter = store.append(iter);
        let value = ProjectTree::value_from_text(name);
        store.set_value(&iter, 0, &value);
        iter
    }
    fn append_folder(&self, store: &TreeStore, iter: &TreeIter,folder: &Folder) {

        for (name, folder) in &folder.folders {
            let iter = ProjectTree::append_name(store, Some(iter), name);
            self.append_folder(store, &iter, folder);
        }
        for name in &folder.files {
            ProjectTree::append_name(store, Some(iter), name);;
        }
    }
    fn value_from_text(text: &str) -> Value {
        unsafe {
                let mut value = Value::new();
                value.init(Type::String);
                value.set_string(text);
                value
            }
    }
    pub fn init_data(&mut self) {
        self.store.clear();
        for pf in &self.core.project.folders {
            let iter = ProjectTree::append_name(&self.store, None, &*pf.name());
            self.append_folder(&self.store, &iter, &pf.folder);
        }
        
    
    }
}
