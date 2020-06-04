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

pub mod id;
pub mod model;
pub mod worker;

use chrono::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{AboutDialog, Application, ApplicationWindow, Builder, Button, Entry, Window};
use storaget::*;

use glib::clone;
use std::cell::RefCell;
use std::env::args;
use std::rc::Rc;

pub use worker::*;

// Contains Worked Edit Dilaog entries
// and their values
struct EditEntries {
    name: gtk::Entry,
    taj: gtk::Entry,
    tax: gtk::Entry,
    mname: gtk::Entry,
    bdate: gtk::Entry,
    bplace: gtk::Entry,
    zip: gtk::Entry,
    city: gtk::Entry,
    street: gtk::Entry,
}

// Build edit dialog
// Returns the dialog itself and the contained entries
fn build_edit_dialog(
    application: &Application,
    glade: &'static str,
    title: &str,
) -> (gtk::Dialog, EditEntries) {
    // Init builder
    let builder = Builder::new_from_string(glade);
    // Build dialog from builder
    let dialog: gtk::Dialog = builder
        .get_object("dialog_edit")
        .expect("Couldn't get dialog edit");

    dialog.set_default_response(gtk::ResponseType::Ok);
    dialog.set_title(&title);

    // Detect entries
    let entries = EditEntries {
        name: builder
            .get_object("entry_name")
            .expect("Failed to load entry name"),
        taj: builder
            .get_object("entry_taj")
            .expect("Failed to load entry taj"),
        tax: builder
            .get_object("entry_tax")
            .expect("Failed to load entry tax"),
        mname: builder
            .get_object("entry_mothersname")
            .expect("Failed to load entry mothersname"),
        bdate: builder
            .get_object("entry_birthdate")
            .expect("Failed to load entry birthdate"),
        bplace: builder
            .get_object("entry_birthplace")
            .expect("Failed to load entry birthplace"),
        zip: builder
            .get_object("entry_zip")
            .expect("Failed to load entry zip"),
        city: builder
            .get_object("entry_city")
            .expect("Failed to load entry city"),
        street: builder
            .get_object("entry_street")
            .expect("Failed to load entry street"),
    };

    let force_alphanumeric = |e: &Entry| {
        e.connect_insert_text(|entry, text, _| {
            if !text.parse::<u32>().is_ok() 
            {
                // entry.delete_text(*p-1, *p);
                // entry.block_signal(???);
                // e.insert_text(&"1", position);
                // entry.unblock_signal(???);
                entry.stop_signal_emission("insert_text");
            }
        });
    };

    force_alphanumeric(&entries.taj);
    force_alphanumeric(&entries.tax);
    force_alphanumeric(&entries.zip);

    // Render save button
    let btn_save: Button = builder.get_object("btn_save").expect("Cannot get btn save");

    // Set response OK when save button clicked
    btn_save
        .connect_clicked(clone!(@weak dialog => move |_| dialog.response(gtk::ResponseType::Ok)));

    // Render cancel button
    let btn_cancel: Button = builder
        .get_object("btn_cancel")
        .expect("Cannot get btn cancel");

    btn_cancel.connect_clicked(clone!(@weak dialog => move |_| {
        dialog.response(gtk::ResponseType::Cancel);
        dialog.destroy();
    }));

    // entry_birthdate
    //     .bind_property("text", &entry_birthplace, "text")
    //     .flags(
    //         glib::BindingFlags::DEFAULT
    //             | glib::BindingFlags::SYNC_CREATE
    //             | glib::BindingFlags::BIDIRECTIONAL,
    //     )
    //     .build();

    // if let Some(id) = worker_id {
    //     let mut data = data.borrow_mut();
    //     let mut _worker = data.as_mut();
    //     let worker: &mut Worker = _worker
    //         .get_worker_mut_by_id(id)
    //         .expect("Failed to get worker by id");

    //     entry_name.set_text(&worker.name);
    //     entry_taj.set_text(&worker.taj);
    //     entry_tax.set_text(&worker.taxnumber);
    //     entry_mothersname.set_text(&worker.mothersname);
    //     entry_birthdate.set_text(&worker.birthdate.to_string());
    //     entry_birthplace.set_text(&worker.birthplace);
    //     entry_zip.set_text(&format!("{}", &worker.zip));
    //     entry_city.set_text(&worker.city);
    //     entry_street.set_text(&worker.street);
    // }

    // btn_save.connect_clicked(clone!(@weak data, @weak dialog => move |_| {
    //     let name = entry_name.get_text().unwrap().to_string();
    //     let taj = entry_taj.get_text().unwrap().to_string();
    //     let tax = entry_tax.get_text().unwrap().to_string();
    //     let mname = entry_mothersname.get_text().unwrap().to_string();
    //     let bdate = entry_birthdate.get_text().unwrap().to_string();
    //     let bplace = entry_birthplace.get_text().unwrap().to_string();
    //     let zip = entry_zip.get_text().unwrap().to_string();
    //     let city = entry_city.get_text().unwrap().to_string();
    //     let street = entry_street.get_text().unwrap().to_string();

    //     let alert = |label: &str| {
    //         let dialog = gtk::DialogBuilder::new().title("Hiba").destroy_with_parent(true).modal(true).build();
    //         let label = gtk::Label::new(Some(label));
    //         label.set_margin_top(19);
    //         label.set_margin_bottom(19);
    //         label.set_margin_start(19);
    //         label.set_margin_end(19);
    //         dialog.get_content_area().add(&label);
    //         dialog.show_all();
    //     };

    //     let date = NaiveDate::parse_from_str(&bdate, "%Y-%m-%d");
    //     if date.is_err() {
    //         alert("A dátum formátuma nem megfelelő!\npl.: 2020-01-01");
    //         return;
    //     }

    //     let _zip = zip.parse::<u32>();

    //     if _zip.is_err() {
    //         alert("Az irányítószám csak számot tartalmazhat!");
    //         return;
    //     }

    //     if let Some(id) = worker_id {
    //         let mut data = data.borrow_mut();
    //         let mut _worker = data.as_mut();
    //         let mut worker: &mut Worker = _worker
    //         .get_worker_mut_by_id(id)
    //         .expect("Failed to get worker by id");

    //         worker.name = name;
    //         worker.taj = taj;
    //         worker.taxnumber = tax;
    //         worker.mothersname = mname;
    //         worker.birthdate = date.unwrap();
    //         worker.birthplace = bplace;
    //         worker.zip = _zip.unwrap();
    //         worker.city = city;
    //         worker.street = street;

    //     } else {
    //         (*data).borrow_mut().as_mut().add_new_worker(name, taj, tax, mname, date.unwrap(), bplace, _zip.unwrap(), city, street).expect("Error while adding new worker");
    //     }
    //     dialog.destroy();
    // }));
    // Add dialog to the main window
    // TODO: is it necessary? As it is a dialog?
    application.add_window(&dialog);
    // Return tuples of the dialog itself
    // and its entries
    (dialog, entries)
}

fn build_ui(application: &gtk::Application, glade: &'static str, db: &Db) {
    // Build ListStore from data
    // We will sync them later by signal handlers
    let model = Rc::new(model::create_model((*db.data).borrow().get_workers()));
    // Build ListStore for selected workers
    // from data
    // We will sync them later by signal handlers
    let model_selected = Rc::new(model::create_model(
        (*db.data).borrow().get_workers_selected(),
    ));
    // Build left treeview
    let treeview_left = gtk::TreeView::new_with_model(&*model);
    treeview_left.set_vexpand(true);
    treeview_left.set_search_column(model::Columns::Name as i32);
    model::add_columns_left(&model, &treeview_left);
    // Build right treeview
    let treeview_right = gtk::TreeView::new_with_model(&*model_selected);
    treeview_right.set_vexpand(true);
    treeview_right.set_search_column(model::Columns::Name as i32);
    model::add_columns_right(&treeview_right);

    // Init builder from static str
    let builder = Builder::new_from_string(glade);
    // Init Application Window
    let window_main: ApplicationWindow = builder
        .get_object("window_main")
        .expect("Couldn't get window");
    // Attach main window to the application
    window_main.set_application(Some(application));

    let btn_about: Button = builder
        .get_object("btn_info")
        .expect("Couldnt get info btn");

    // Build About dialog when Info button clicked
    btn_about.connect_clicked(
        move |button| {
            let builder = Builder::new_from_string(glade);
            let dialog: AboutDialog = builder
                .get_object("window_about")
                .expect("Error loading about window");
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
    );

    // Build left panel
    let left_panel: gtk::Box = builder
        .get_object("left_panel")
        .expect("Cannot get main box");

    // Build right panel
    let right_panel: gtk::Box = builder
        .get_object("right_panel")
        .expect("Cannot get main box");

    // Create a scrolled window
    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);

    // Add scrolled window to the left panel
    left_panel.add(&sw);

    // Add treeview_left to the scrolled window LEFT
    sw.add(&treeview_left);

    // Create another scrolled window
    let sw2 = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw2.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw2.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);

    // Add scrolled window to the right panel
    right_panel.add(&sw2);

    // Add treview_right to the scrolled window RIGHT
    sw2.add(&treeview_right);

    // Build NEW button
    let btn_new: Button = builder
        .get_object("btn_new")
        .expect("Couldnt get new button");

    let data = db.data.clone();
    // New button click action
    // Open new_or_edit dialog
    btn_new.connect_clicked(clone!(@weak application, @weak data, @strong model => move |_| {
        let (dialog, entries) = build_edit_dialog(&application, glade, "Új munkavállaló hozzáadása");
        dialog.connect_response(clone!(@weak model => move |dialog, resp| {
            if resp == gtk::ResponseType::Ok {
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
                let date = NaiveDate::parse_from_str(&entries.bdate.get_text().unwrap().to_string(), "%Y-%m-%d");
                if date.is_err() {
                    alert("A dátum formátuma nem megfelelő!\npl.: 2020-01-01");
                    return;
                }

                let _zip = entries.zip.get_text().unwrap().to_string().parse::<u32>();

                if _zip.is_err() {
                    alert("Az irányítószám csak számot tartalmazhat!");
                    return;
                }

                // Store data to Pack storage
                let id = (*data).borrow_mut()
                    .as_mut()
                    .add_new_worker(
                        entries.name.get_text().unwrap().to_string(),
                        entries.taj.get_text().unwrap().to_string(),
                        entries.tax.get_text().unwrap().to_string(),
                        entries.mname.get_text().unwrap().to_string(),
                        date.unwrap(),
                        entries.bplace.get_text().unwrap().to_string(),
                        _zip.unwrap(),
                        entries.city.get_text().unwrap().to_string(),
                        entries.street.get_text().unwrap().to_string());
                
                let col_indices: [u32; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
                let values: [&dyn ToValue; 11] = [
                    &id.unwrap(),
                    &entries.name.get_text().unwrap().to_string(),
                    &entries.mname.get_text().unwrap().to_string(),
                    &entries.bdate.get_text().unwrap().to_string(),
                    &entries.bplace.get_text().unwrap().to_string(),
                    &entries.zip.get_text().unwrap().to_string().parse::<u32>().expect("Error while casting ZIP from string to u32"),
                    &entries.city.get_text().unwrap().to_string(),
                    &entries.street.get_text().unwrap().to_string(),
                    &false,
                    &entries.tax.get_text().unwrap().to_string(),
                    &entries.taj.get_text().unwrap().to_string(),
                ];
                model.set(&model.append(), &col_indices, &values);
            }
            dialog.destroy();
        }));
        // Display dialog
        dialog.show_all();
    }));

    model.connect_row_changed(clone!(@weak data, @strong model_selected => move |s, path, iter| {
        let id = s
            .get_value(iter, model::Columns::Id as i32)
            .get_some::<u32>()
            .unwrap();
        let name = s
            .get_value(iter, model::Columns::Name as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let taj = s
            .get_value(iter, model::Columns::TAJ as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let tax = s
            .get_value(iter, model::Columns::Tax as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let mname = s
            .get_value(iter, model::Columns::Mname as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let name = s
            .get_value(iter, model::Columns::Name as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let bdate = s
            .get_value(iter, model::Columns::Birthdate as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let bplace = s
            .get_value(iter, model::Columns::Birthplace as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let zip = s
            .get_value(iter, model::Columns::Zip as i32)
            .get::<u32>()
            .unwrap()
            .unwrap();
        let city = s
            .get_value(iter, model::Columns::City as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let street = s
            .get_value(iter, model::Columns::Street as i32)
            .get::<String>()
            .unwrap()
            .unwrap();
        let is_selected = s
            .get_value(&iter, model::Columns::IsSelected as i32)
            .get_some::<bool>()
            .unwrap();

        let worker = Worker::new(id,
            name,
            taj,
            tax,
            mname,
            NaiveDate::parse_from_str(&bdate, "%Y-%m-%d").expect("Error while convertin date to NaiveDate from string, on row change"),
            bplace,
            zip,
            city,
            street,
            is_selected
        );

        // Store data to Pack storage
        (*data).borrow_mut()
            .as_mut()
            .update_worker_by_id(worker, id).expect("Error while updating worker Pack storage");

        model::update_model(&*model_selected, (*data).borrow().get_workers_selected());
        
        println!("Row changed! Id {}", id);
    }));

    let data = db.data.clone();
    let m = model.clone();
    // Left douple click & enter action
    treeview_left.connect_row_activated(clone!(@weak application, @weak data, @weak treeview_left, @strong model as _model => move |a, b, _| {
        let model = a.get_model().unwrap();
        let iter = model.get_iter(b).unwrap();

        let (dialog, entries) = build_edit_dialog(&application, glade, "Munkavállaló szerkesztése");
        
        let id = model
            .get_value(&iter, model::Columns::Id as i32)
            .get_some::<u32>()
            .unwrap();

        let is_selected = model
            .get_value(&iter, model::Columns::IsSelected as i32)
            .get_some::<bool>()
            .unwrap();

        entries.name.set_text(&model
            .get_value(&iter, model::Columns::Name as i32).get::<String>().unwrap().unwrap());
        entries.mname.set_text(&model
            .get_value(&iter, model::Columns::Mname as i32).get::<String>().unwrap().unwrap());
        entries.taj.set_text(&model
            .get_value(&iter, model::Columns::TAJ as i32).get::<String>().unwrap().unwrap());
        entries.tax.set_text(&model
            .get_value(&iter, model::Columns::Tax as i32).get::<String>().unwrap().unwrap());
        entries.bplace.set_text(&model
            .get_value(&iter, model::Columns::Birthplace as i32).get::<String>().unwrap().unwrap());
        entries.bdate.set_text(&model
            .get_value(&iter, model::Columns::Birthdate as i32).get::<String>().unwrap().unwrap());
        entries.zip.set_text(&model
            .get_value(&iter, model::Columns::Zip as i32).get::<u32>().unwrap().unwrap().to_string());
        entries.city.set_text(&model
            .get_value(&iter, model::Columns::City as i32).get::<String>().unwrap().unwrap().to_string());
        entries.street.set_text(&model
            .get_value(&iter, model::Columns::Street as i32).get::<String>().unwrap().unwrap().to_string());

            dialog.connect_response(clone!(@weak _model => move |dialog, resp| {
                if resp == gtk::ResponseType::Ok {
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
                    let date = NaiveDate::parse_from_str(&entries.bdate.get_text().unwrap().to_string(), "%Y-%m-%d");
                    if date.is_err() {
                        alert("A dátum formátuma nem megfelelő!\npl.: 2020-01-01");
                        return;
                    }
    
                    let _zip = entries.zip.get_text().unwrap().to_string().parse::<u32>();
    
                    if _zip.is_err() {
                        alert("Az irányítószám csak számot tartalmazhat!");
                        return;
                    }

                    // let worker = Worker::new(id,
                    //     entries.name.get_text().unwrap().to_string(),
                    //     entries.taj.get_text().unwrap().to_string(),
                    //     entries.tax.get_text().unwrap().to_string(),
                    //     entries.mname.get_text().unwrap().to_string(),
                    //     date.unwrap(),
                    //     entries.bplace.get_text().unwrap().to_string(),
                    //     _zip.unwrap(),
                    //     entries.city.get_text().unwrap().to_string(),
                    //     entries.street.get_text().unwrap().to_string(),
                    //     is_selected
                    // );
    
                    // // Store data to Pack storage
                    // (*data).borrow_mut()
                    //     .as_mut()
                    //     .update_worker_by_id(worker, id).expect("Error while updating worker Pack storage");
                    
                    let col_indices: [u32; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
                    let values: [&dyn ToValue; 11] = [
                        &id,
                        &entries.name.get_text().unwrap().to_string(),
                        &entries.mname.get_text().unwrap().to_string(),
                        &entries.bdate.get_text().unwrap().to_string(),
                        &entries.bplace.get_text().unwrap().to_string(),
                        &entries.zip.get_text().unwrap().to_string().parse::<u32>().expect("Error while casting ZIP from string to u32"),
                        &entries.city.get_text().unwrap().to_string(),
                        &entries.street.get_text().unwrap().to_string(),
                        &false,
                        &entries.tax.get_text().unwrap().to_string(),
                        &entries.taj.get_text().unwrap().to_string(),
                    ];
                    _model.set(&iter, &col_indices, &values);
                }
                dialog.destroy();
            }));

        dialog.show_all();
        // let change_to = match is_selected {
        //     true => false,
        //     false => true,
        // };
        // {
        //     let _ = data.borrow_mut().as_mut().set_worker_selected_by_id(id, change_to);
        // }

        // let tv: &dyn ToValue = &change_to;
        // m.set_value(&iter, Columns::IsSelected as u32, &tv.to_value());
    }));

//     let m2 = model.clone();
//     treeview.connect_key_press_event(
//         clone!(@weak window_main => @default-return Inhibit(false), move |treeview, event| {
//             // If del pressed
//             if event.get_hardware_keycode() == 119 {
//                 let dialog = gtk::Dialog::new_with_buttons(
//                     Some("Biztosan törlöd?"),
//                     Some(&window_main),
//                     gtk::DialogFlags::MODAL,
//                     &[
//                         ("Törlés", gtk::ResponseType::Ok),
//                         ("Mégsem", gtk::ResponseType::Cancel),
//                     ],
//                 );
//                 dialog.set_default_response(gtk::ResponseType::Ok);
//                 let label = gtk::Label::new(Some("Biztosan törlöd\na kiválasztott munkavállalót?"));
//                 label.set_justify(gtk::Justification::Center);
//                 label.set_margin_start(19);
//                 label.set_margin_end(19);
//                 label.set_margin_top(19);
//                 label.set_margin_bottom(19);
//                 dialog.get_content_area().add(&label);

//                 dialog.connect_response(
//                     clone!(@weak treeview, @weak data, @weak m2 => move |dialog, resp| {
//                         if resp == gtk::ResponseType::Ok {
//                             let (model, iter) = treeview.get_selection().get_selected().unwrap();
//                             let id = model
//                                 .get_value(&iter, Columns::Id as i32)
//                                 .get_some::<u32>()
//                                 .unwrap();
//                             // Try to remove worker from Pack by ID
//                             if let Some(_) = data.borrow_mut().as_mut().remove_worker_by_id(id) {
//                                 // If success, then remove from liststore as well
//                                 (*m2).remove(&iter);
//                             }
//                         }
//                         dialog.destroy();
//                     }),
//                 );
//                 dialog.show_all();
//             }
//             gtk::Inhibit(false)
//         }),
//     );
//     // Right click action
//     treeview.connect_button_press_event(move |treeview, event| {
//         if event.get_event_type() == gdk::EventType::ButtonPress && event.get_button() == 3 {
//             let (x, y) = event.get_coords().expect("Couldnt get click coordinates");
//             let (path, _, _, _) = treeview
//                 .get_path_at_pos(x as i32, y as i32)
//                 .expect("Error while getting path at pos");
//             let model = treeview.get_model().unwrap();
//             let iter = model.get_iter(&path.unwrap()).unwrap();
//             let id = model
//                 .get_value(&iter, Columns::Id as i32)
//                 .get_some::<u32>()
//                 .unwrap();
//             println!("Right click at id {}", id);
//         }
//         Inhibit(false)
//     });

    window_main.show_all();
}

// fn update_right_panel(treeview: &gtk::TreeView, data: &Data) {
//     let model_selected = Rc::new(create_model(data.get_workers_selected()));
//     treeview.set_vexpand(true);
//     treeview.set_model(Some(&*model_selected));
// }

fn refresh_treeview(treeview: &gtk::TreeView, model: &gtk::ListStore) {
    treeview.set_model(Some(model));
}

// Application DB
struct Db {
    data: Rc<RefCell<Pack<Data>>>,
}

fn main() {
    // Init Application DB
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

    let application = gtk::Application::new(Some("com.dailyworker"), Default::default())
        .expect("Initialization failed...");

    let glade_src = include_str!("../data/ui/design.glade");

    application.connect_activate(move |app| build_ui(app, &glade_src, &db));

    application.run(&args().collect::<Vec<_>>());
}
