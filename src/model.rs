use crate::Worker;
use gio::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
#[repr(i32)]
pub enum Columns {
    Id,
    Name,
    Mname,
    Birthdate,
    Birthplace,
    Zip,
    City,
    Street,
    IsSelected,
    Tax,
    TAJ,
}

pub fn create_model(workers: Vec<&Worker>) -> gtk::ListStore {
    let col_types: [glib::Type; 11] = [
        glib::Type::U32,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::U32,
        glib::Type::String,
        glib::Type::String,
        glib::Type::Bool,
        glib::Type::String,
        glib::Type::String,
    ];

    let store = gtk::ListStore::new(&col_types);

    let col_indices: [u32; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for (d_idx, w) in workers.iter().enumerate() {
        let values: [&dyn ToValue; 11] = [
            &w.id,
            &w.name,
            &w.mothersname,
            &w.birthdate.to_string(),
            &w.birthplace,
            &w.zip,
            &w.city,
            &w.street,
            &w.is_selected,
            &w.taxnumber,
            &w.taj,
        ];
        store.set(&store.append(), &col_indices, &values);
    }

    store
}

pub fn update_model(store: &gtk::ListStore, workers: Vec<&Worker>) {
    let col_types: [glib::Type; 11] = [
        glib::Type::U32,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::U32,
        glib::Type::String,
        glib::Type::String,
        glib::Type::Bool,
        glib::Type::String,
        glib::Type::String,
    ];

    // let store = gtk::ListStore::new(&col_types);
    store.clear();

    let col_indices: [u32; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for (d_idx, w) in workers.iter().enumerate() {
        let values: [&dyn ToValue; 11] = [
            &w.id,
            &w.name,
            &w.mothersname,
            &w.birthdate.to_string(),
            &w.birthplace,
            &w.zip,
            &w.city,
            &w.street,
            &w.is_selected,
            &w.taxnumber,
            &w.taj,
        ];
        store.set(&store.append(), &col_indices, &values);
    }
}

fn fixed_toggled<W: IsA<gtk::CellRendererToggle>>(
    model: &gtk::ListStore,
    _w: &W,
    path: gtk::TreePath,
) {
    let iter = model.get_iter(&path).unwrap();
    let mut fixed = model
        .get_value(&iter, Columns::IsSelected as i32)
        .get_some::<bool>()
        .unwrap_or_else(|err| {
            panic!(
                "ListStore value for {:?} at path {}: {}",
                Columns::IsSelected,
                path,
                err
            )
        });
    fixed = !fixed;
    model.set_value(&iter, Columns::IsSelected as u32, &fixed.to_value());
}

pub fn add_columns_left(model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {
    // Column for fixed toggles
    {
        let renderer = gtk::CellRendererToggle::new();
        // renderer.set_activatable(false);
        let model_clone = model.clone();
        renderer.connect_toggled(move |w, path| fixed_toggled(&model_clone, w, path));
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("?");
        column.add_attribute(&renderer, "active", Columns::IsSelected as i32);
        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        column.set_fixed_width(50);
        column.set_sort_column_id(Columns::IsSelected as i32);
        treeview.append_column(&column);
    }
    // Column for name
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Név");
        column.add_attribute(&renderer, "text", Columns::Name as i32);
        column.set_sort_column_id(Columns::Name as i32);
        treeview.append_column(&column);
    }
    // Column for birthdate
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Születési dátum");
        column.add_attribute(&renderer, "text", Columns::Birthdate as i32);
        column.set_sort_column_id(Columns::Birthdate as i32);
        treeview.append_column(&column);
    }
    // Column for city
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Település");
        column.add_attribute(&renderer, "text", Columns::City as i32);
        column.set_sort_column_id(Columns::City as i32);
        treeview.append_column(&column);
    }
    // Column for street
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Utca, házszám");
        column.add_attribute(&renderer, "text", Columns::Street as i32);
        column.set_sort_column_id(Columns::Street as i32);
        treeview.append_column(&column);
    }
    // Column for taxnumber
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Adóazonosító");
        column.add_attribute(&renderer, "text", Columns::Tax as i32);
        column.set_sort_column_id(Columns::Tax as i32);
        treeview.append_column(&column);
    }
    // Column for TAJ
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Tajszám");
        column.add_attribute(&renderer, "text", Columns::TAJ as i32);
        column.set_sort_column_id(Columns::TAJ as i32);
        treeview.append_column(&column);
    }
}

pub fn add_columns_right(treeview: &gtk::TreeView) {
    // Column for name
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Név");
        column.add_attribute(&renderer, "text", Columns::Name as i32);
        column.set_sort_column_id(Columns::Name as i32);
        treeview.append_column(&column);
    }
    // Column for birthdate
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Születési dátum");
        column.add_attribute(&renderer, "text", Columns::Birthdate as i32);
        column.set_sort_column_id(Columns::Birthdate as i32);
        treeview.append_column(&column);
    }
    // Column for city
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Település");
        column.add_attribute(&renderer, "text", Columns::City as i32);
        column.set_sort_column_id(Columns::City as i32);
        treeview.append_column(&column);
    }
    // Column for TAJ
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Tajszám");
        column.add_attribute(&renderer, "text", Columns::TAJ as i32);
        column.set_sort_column_id(Columns::TAJ as i32);
        treeview.append_column(&column);
    }
}
