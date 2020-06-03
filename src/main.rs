extern crate cairo;
extern crate chrono;
extern crate dirs;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate nanoid;
extern crate pango;
extern crate pangocairo;
extern crate storaget;

use chrono::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{AboutDialog, Application, ApplicationWindow, Builder, Button, Entry, Window};
use serde::{Deserialize, Serialize};
use storaget::*;

use glib::clone;
use std::cell::RefCell;
use std::env::args;
use std::rc::Rc;

mod id;

fn about(button: &Button, dialog: &AboutDialog) {
    if let Some(window) = button
        .get_toplevel()
        .and_then(|w| w.downcast::<Window>().ok())
    {
        dialog.set_transient_for(Some(&window));
    }

    // Since we only have once instance of this object with Glade, we only show/hide it.
    dialog.show();
    dialog.run();
    dialog.hide();
}

fn edit(application: &Application, glade: &'static str) {
    let builder = Builder::new_from_string(glade);
    let window: Window = builder
        .get_object("window_edit")
        .expect("Couldn't get window edit");
    application.add_window(&window);
    window.show_all();
}

fn new(
    application: &Application,
    glade: &'static str,
    data: &Rc<RefCell<Pack<Data>>>,
    treeview: &gtk::TreeView,
) {
    let builder = Builder::new_from_string(glade);
    let window: Window = builder
        .get_object("window_edit")
        .expect("Couldn't get window edit");
    let btn_save: Button = builder.get_object("btn_save").expect("Cannot get btn save");
    let entry_name: Entry = builder
        .get_object("entry_name")
        .expect("Failed to load entry name");
    let entry_taj: Entry = builder
        .get_object("entry_taj")
        .expect("Failed to load entry taj");
    let entry_tax: Entry = builder
        .get_object("entry_tax")
        .expect("Failed to load entry tax");
    let entry_mothersname: Entry = builder
        .get_object("entry_mothersname")
        .expect("Failed to load entry mothersname");
    let entry_birthdate: Entry = builder
        .get_object("entry_birthdate")
        .expect("Failed to load entry birthdate");
    let entry_birthplace: Entry = builder
        .get_object("entry_birthplace")
        .expect("Failed to load entry birthplace");
    let entry_zip: Entry = builder
        .get_object("entry_zip")
        .expect("Failed to load entry zip");
    let entry_city: Entry = builder
        .get_object("entry_city")
        .expect("Failed to load entry city");
    let entry_street: Entry = builder
        .get_object("entry_street")
        .expect("Failed to load entry street");
    btn_save.connect_clicked(clone!(@weak data, @weak window, @weak treeview => move |_| {
        let name = entry_name.get_text().unwrap().to_string();
        let taj = entry_taj.get_text().unwrap().to_string();
        let tax = entry_tax.get_text().unwrap().to_string();
        let mname = entry_mothersname.get_text().unwrap().to_string();
        let bdate = entry_birthdate.get_text().unwrap().to_string();
        let bplace = entry_birthplace.get_text().unwrap().to_string();
        let zip = entry_zip.get_text().unwrap().to_string();
        let city = entry_city.get_text().unwrap().to_string();
        let street = entry_street.get_text().unwrap().to_string();

        let alert = |label: &str| {
            let dialog = gtk::DialogBuilder::new().title("Hiba").destroy_with_parent(true).modal(true).build();
            let label = gtk::Label::new(Some(label));
            label.set_margin_top(19);
            label.set_margin_bottom(19);
            label.set_margin_start(19);
            label.set_margin_end(19);
            dialog.get_content_area().add(&label);
            dialog.show_all();
        };

        let date = NaiveDate::parse_from_str(&bdate, "%Y-%m-%d");
        if date.is_err() {
            alert("A dátum formátuma nem megfelelő!\npl.: 2020-01-01");
            return;
        }

        let _zip = zip.parse::<u32>();

        if _zip.is_err() {
            alert("Az irányítószám csak számot tartalmazhat!");
            return;
        }

        (*data).borrow_mut().as_mut().add_new_worker(name, taj, tax, mname, date.unwrap(), bplace, _zip.unwrap(), city, street).expect("Error while adding new worker");

        refresh_treeview(&treeview, &create_model((*data).borrow().get_workers()));
        window.destroy();
    }));
    application.add_window(&window);
    window.show_all();
}

fn build_ui(application: &gtk::Application, glade: &'static str, db: &Db) {
    let builder = Builder::new_from_string(glade);
    let window_main: ApplicationWindow = builder
        .get_object("window_main")
        .expect("Couldn't get window");
    window_main.set_application(Some(application));

    let dialog_about: AboutDialog = builder
        .get_object("window_about")
        .expect("Error loading about window");

    let btn_info: Button = builder
        .get_object("btn_info")
        .expect("Couldnt get info btn");

    btn_info.connect_clicked(move |x| about(x, &dialog_about));

    let btn_new: Button = builder.get_object("btn_new").expect("Couldnt get info new");
    let data = db.data.clone();

    let main_box: gtk::Box = builder.get_object("main_box").expect("Cannot get main box");

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    main_box.add(&sw);

    let model = Rc::new(create_model((*data).borrow().get_workers()));
    let treeview = gtk::TreeView::new_with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_search_column(Columns::Name as i32);

    sw.add(&treeview);

    add_columns(&model, &treeview);

    btn_new.connect_clicked(clone!(@weak application, @weak data => move |_| {
        new(&application, glade, &data, &treeview);
    }));

    window_main.show_all();
}

fn refresh_treeview(treeview: &gtk::TreeView, model: &gtk::ListStore) {
    treeview.set_model(Some(model));
}

struct TableData {
    name: String,
    bdate: String,
    city: String,
    street: String,
}

fn create_model(workers: Vec<&Worker>) -> gtk::ListStore {
    let col_types: [glib::Type; 4] = [
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
    ];

    let store = gtk::ListStore::new(&col_types);

    let col_indices: [u32; 4] = [0, 1, 2, 3];

    for (d_idx, w) in workers.iter().enumerate() {
        let values: [&dyn ToValue; 4] = [&w.name, &w.birthdate.to_string(), &w.city, &w.street];
        store.set(&store.append(), &col_indices, &values);
    }

    store
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Name,
    Birthdate,
    City,
    Street,
}

fn add_columns(model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {
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
        column.set_title("City");
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    employers: Vec<Employer>,
    employer_counter: u32,
    worker_counter: u32,
    workers: Vec<Worker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Employer {
    id: u32,
    name: String,
    taxnumber: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Worker {
    id: u32,
    name: String,
    taj: String,
    taxnumber: String,
    mothersname: String,
    birthdate: NaiveDate,
    birthplace: String,
    zip: u32,
    city: String,
    street: String,
    is_selected: bool,
}

impl Data {
    pub fn add_new_worker(
        &mut self,
        name: String,
        taj: String,
        taxnumber: String,
        mothersname: String,
        birthdate: NaiveDate,
        birthplace: String,
        zip: u32,
        city: String,
        street: String,
    ) -> Result<(), String> {
        self.worker_counter += 1;
        let new_worker = Worker {
            id: self.worker_counter,
            name,
            taj,
            taxnumber,
            mothersname,
            birthdate,
            birthplace,
            zip,
            city,
            street,
            is_selected: false,
        };
        self.workers.push(new_worker);
        Ok(())
    }
    pub fn update_worker_by_id(&mut self, new_worker: Worker, id: u32) -> Result<&Worker, String> {
        for worker in &mut self.workers {
            if worker.id == id {
                *worker = new_worker;
                return Ok(worker);
            }
        }
        Err(format!("Worker not found by ID"))
    }
    pub fn get_worker_mut_by_id(&mut self, id: u32) -> Option<&mut Worker> {
        for worker in &mut self.workers {
            if worker.id == id {
                return Some(worker);
            }
        }
        None
    }
    pub fn get_workers(&self) -> Vec<&Worker> {
        self.workers.iter().map(|w| w).collect::<Vec<&Worker>>()
    }
    pub fn get_workers_selected(&self) -> Vec<&Worker> {
        self.workers
            .iter()
            .filter(|w| w.is_selected)
            .map(|w| w)
            .collect::<Vec<&Worker>>()
    }
    pub fn add_new_employer(&mut self, name: String, taxnumber: String) {
        self.employer_counter += 1;
        let new_employer = Employer {
            id: self.employer_counter,
            name,
            taxnumber,
        };
        self.employers.push(new_employer);
    }
    pub fn get_employers(&self) -> &Vec<Employer> {
        &self.employers
    }
    pub fn get_employer_mut_by_id(&mut self, id: u32) -> Option<&mut Employer> {
        for employer in &mut self.employers {
            if employer.id == id {
                return Some(employer);
            }
        }
        None
    }
}

impl Default for Data {
    fn default() -> Self {
        Data {
            employers: Vec::new(),
            employer_counter: 0,
            worker_counter: 0,
            workers: Vec::new(),
        }
    }
}

impl TryFrom for Data {
    type TryFrom = Data;
}

struct Db {
    data: Rc<RefCell<Pack<Data>>>,
}

fn main() {
    let db = Db {
        data: Rc::new(RefCell::new(
            Pack::try_load_or_init(
                dirs::home_dir()
                    .expect("Error while getting your home folder")
                    .join(".dailyworkerdb"),
                "workersdb",
            )
            .expect("Error loading workers db"),
        )),
    };

    let application = gtk::Application::new(Some("com.labelprinting"), Default::default())
        .expect("Initialization failed...");

    let glade_src = include_str!("../data/ui/design.glade");

    application.connect_activate(move |app| build_ui(app, &glade_src, &db));

    application.run(&args().collect::<Vec<_>>());
}
