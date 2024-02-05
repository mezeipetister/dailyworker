use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use chrono::prelude::*;
use dirs::home_dir;
use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_workers_dir() -> Result<PathBuf, String> {
    if let Some(dir) = home_dir() {
        let data_dir = dir.join(".dailyworkerdata");
        // Check if exist
        if !data_dir.exists() {
            // Create path if needed
            let _ = std::fs::create_dir_all(&data_dir);
        }
        let workers_dir = data_dir.join("workers");
        // Check if exist
        if !workers_dir.exists() {
            // Create path if needed
            let _ = std::fs::create_dir_all(&workers_dir);
        }
        return Ok(workers_dir);
    }
    Err("Error while getting context".into())
}

#[derive(Debug)]
pub(crate) struct Db {
    pub(crate) workers: Vec<Worker>,
}

impl Db {
    pub(crate) fn open() -> Result<Self, String> {
        let mut workers: Vec<Worker> = vec![];
        let files = std::fs::read_dir(&get_workers_dir()?).map_err(|e| e.to_string())?;
        for file in files {
            if let Ok(dir_entry) = file {
                let mut file = File::open(&dir_entry.path()).map_err(|e| e.to_string())?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)
                    .map_err(|e| e.to_string())?;
                let worker: Worker = serde_json::from_str(&buffer).map_err(|e| e.to_string())?;
                workers.push(worker);
            }
        }
        workers.sort_by(|a, b| a.name.cmp(&b.name));
        let res = Self { workers };
        Ok(res)
    }
    pub(crate) fn add_new_worker(&mut self, worker: Worker) -> Result<(), String> {
        worker.save()?;
        self.workers.push(worker);
        Ok(())
    }
    pub(crate) fn remove_worker(&mut self, worker: Worker) -> Result<(), String> {
        // Remove from storage
        worker.delete()?;
        // Remove from DB
        self.workers.retain(|w| w.id != worker.id);
        Ok(())
    }
    pub(crate) fn update_worker(&mut self, new_worker: Worker) -> Result<&Worker, String> {
        for worker in &mut self.workers {
            if worker.id == new_worker.id {
                new_worker.save()?;
                *worker = new_worker;
                return Ok(worker);
            }
        }
        Err(format!("Worker not found by ID"))
    }
    pub(crate) fn set_worker_selected_by_id(
        &mut self,
        id: Uuid,
        selected: bool,
    ) -> Option<&mut Worker> {
        for worker in &mut self.workers {
            if worker.id == id {
                worker.is_selected = selected;
                let _ = worker.save();
                return Some(worker);
            }
        }
        None
    }
    pub(crate) fn get_by_id(&self, id: Uuid) -> Option<&Worker> {
        self.workers.iter().find(|w| w.id == id)
    }
    pub(crate) fn get_workers_selected(&self) -> Vec<&Worker> {
        self.workers
            .iter()
            .filter(|w| w.is_selected)
            .map(|w| w)
            .collect::<Vec<&Worker>>()
    }
}

#[frb(non_final)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Worker {
    pub id: Uuid,
    #[frb(non_final)]
    pub name: String,
    #[frb(non_final)]
    pub taj: String,
    #[frb(non_final)]
    pub taxnumber: String,
    #[frb(non_final)]
    pub mothersname: String,
    #[frb(non_final)]
    pub birthdate: String,
    #[frb(non_final)]
    pub birthplace: String,
    #[frb(non_final)]
    pub zip: String,
    #[frb(non_final)]
    pub city: String,
    #[frb(non_final)]
    pub street: String,
    #[frb(non_final)]
    pub is_selected: bool,
}

impl Default for Worker {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: Default::default(),
            taj: Default::default(),
            taxnumber: Default::default(),
            mothersname: Default::default(),
            birthdate: Default::default(),
            birthplace: Default::default(),
            zip: Default::default(),
            city: Default::default(),
            street: Default::default(),
            is_selected: Default::default(),
        }
    }
}

impl Worker {
    pub(crate) fn new(
        name: String,
        taj: String,
        taxnumber: String,
        mothersname: String,
        birthdate: String,
        birthplace: String,
        zip: String,
        city: String,
        street: String,
        is_selected: bool,
    ) -> Self {
        Worker {
            id: Uuid::new_v4(),
            name,
            taj,
            taxnumber,
            mothersname,
            birthdate,
            birthplace,
            zip,
            city,
            street,
            is_selected,
        }
    }

    #[frb(sync)]
    pub fn cloned(&self) -> Worker {
        self.to_owned()
    }

    #[frb(sync)]
    pub fn set_selected(&self, to: bool) -> Worker {
        let mut n = self.to_owned();
        n.is_selected = to;
        n
    }

    #[frb(sync)]
    pub fn has_valid_birthdate(&self) -> bool {
        NaiveDate::parse_from_str(&self.birthdate, "%Y-%m-%d").is_ok()
    }
    pub(crate) fn save(&self) -> Result<&Self, String> {
        let file_path = get_workers_dir()?.join(format!("{}.json", self.id.as_simple()));
        let mut buffer = BufWriter::new(File::create(&file_path).map_err(|e| e.to_string())?);
        buffer
            .write_all(
                serde_json::to_string(&self)
                    .map_err(|e| e.to_string())?
                    .as_bytes(),
            )
            .map_err(|e| e.to_string())?;
        buffer.flush().map_err(|e| e.to_string())?;
        Ok(self)
    }
    pub(crate) fn delete(&self) -> Result<(), String> {
        let file_path = get_workers_dir()?.join(format!("{}.json", self.id.as_simple()));
        std::fs::remove_file(&file_path).map_err(|e| e.to_string())
    }
}
