extern crate gtk;

use gtk::prelude::*;
use gtk::{Builder, ImageMenuItem, Window, TreeView, ListStore, TreeViewColumn};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("basic.glade");
    let builder = Builder::new_from_string(glade_src);
    let main_window: Window = builder.get_object("MainWindow").unwrap();
    let tree_view: TreeView = builder.get_object("TreeView").unwrap();
    let quit_menu_item: ImageMenuItem = builder.get_object("beendenMenuItem").unwrap();

    let types_inside_columns = &[gtk::Type::String];
    let model_list_of_data = ListStore::new(types_inside_columns);
    tree_view.set_model(Some(&model_list_of_data));
    // Column 1
    let view_column_1 = TreeViewColumn::new();
    view_column_1.set_title("Feiertag");
    tree_view.append_column(&view_column_1);

    model_list_of_data.clear();
    let row = model_list_of_data.append();
    model_list_of_data.set_value(&row, 0, &"Test1".to_string().to_value());

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    quit_menu_item.connect_activate(|_| {
        gtk::main_quit();
    });

    main_window.show_all();
    gtk::main();
}
