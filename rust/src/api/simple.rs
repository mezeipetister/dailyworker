use std::{
    fs::File,
    io::Write,
    path::Path,
    sync::{OnceLock, RwLock},
};

use chrono::Local;

use crate::xml::render_xml;

use super::worker::{Db, Worker};

static COUNTER: OnceLock<RwLock<i32>> = OnceLock::new();
static DB: OnceLock<RwLock<crate::api::worker::Db>> = OnceLock::new();

fn _export_xml(to: String) -> Result<(), String> {
    let selected_workers = _get_workers()
        .into_iter()
        .filter(|w| w.is_selected)
        .collect::<Vec<Worker>>();

    let xml = render_xml(selected_workers);

    let file_name = format!("{}.xml", Local::now().naive_local().to_string());

    let path = Path::new(&to).join(&file_name);

    let mut file = File::create(path).map_err(|e| e.to_string())?;

    file.write_all(xml.as_bytes()).map_err(|e| e.to_string())?;

    file.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[flutter_rust_bridge::frb(sync)]
pub fn export_xml_api(to: String) -> Result<(), String> {
    _export_xml(to)
}

fn _get_workers() -> Vec<Worker> {
    let mut db = DB.get().unwrap().read().unwrap().workers.to_owned();
    db.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    db
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_workers() -> Vec<Worker> {
    _get_workers()
}

fn _add_worker(worker: Worker) {
    DB.get()
        .unwrap()
        .write()
        .unwrap()
        .add_new_worker(worker)
        .unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn add_worker(worker: Worker) {
    _add_worker(worker)
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_empty_worker() -> Worker {
    let w = Worker::default();
    println!("{:?}", &w);
    w
}

fn _update_worker(worker: Worker) {
    DB.get()
        .unwrap()
        .write()
        .unwrap()
        .update_worker(worker)
        .unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn update_worker(worker: Worker) {
    _update_worker(worker)
}

fn _remove_worker(worker: Worker) {
    DB.get()
        .unwrap()
        .write()
        .unwrap()
        .remove_worker(worker)
        .unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn remove_worker_api(worker: Worker) {
    _remove_worker(worker)
}

fn _init() {
    COUNTER.set(RwLock::new(0)).unwrap();
    DB.set(RwLock::new(Db::open().unwrap())).unwrap();
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
    _init()
}
