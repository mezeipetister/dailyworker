use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
    str::FromStr,
};

use chrono::prelude::*;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Context {
    data_dir: PathBuf,
    workers_dir: PathBuf,
}

impl Context {
    pub fn new() -> Result<Self, String> {
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
            return Ok(Self {
                data_dir,
                workers_dir,
            });
        }
        Err("Error while getting context".into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Worker {
    pub id: Uuid,
    pub name: String,
    pub taj: String,
    pub taxnumber: String,
    pub mothersname: String,
    pub birthdate: String,
    pub birthplace: String,
    pub zip: String,
    pub city: String,
    pub street: String,
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
    pub fn new(
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
    pub fn has_valid_birthdate(&self) -> bool {
        NaiveDate::parse_from_str(&self.birthdate, "%Y-%m-%d").is_ok()
    }
    pub fn save(&self, ctx: &Context) -> Result<&Self, String> {
        let file_path = ctx
            .workers_dir
            .join(format!("{}.json", self.id.as_simple()));
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
}

#[derive(Debug, Clone, Default)]
pub struct Data {
    workers: Vec<Worker>,
    ctx: Context,
}

impl Data {
    pub fn init(ctx: Context) -> Result<Self, String> {
        let mut workers: Vec<Worker> = vec![];
        let files = std::fs::read_dir(&ctx.workers_dir).map_err(|e| e.to_string())?;
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
        let res = Self { workers, ctx };
        Ok(res)
    }
    pub fn add_new_worker(&mut self, worker: Worker) -> Result<(), String> {
        worker.save(&self.ctx)?;
        self.workers.push(worker);
        Ok(())
    }
    pub fn update_worker(&mut self, new_worker: Worker) -> Result<&Worker, String> {
        for worker in &mut self.workers {
            if worker.id == new_worker.id {
                new_worker.save(&self.ctx)?;
                *worker = new_worker;
                return Ok(worker);
            }
        }
        Err(format!("Worker not found by ID"))
    }
    pub fn set_worker_selected_by_id(&mut self, id: Uuid, selected: bool) -> Option<&mut Worker> {
        for worker in &mut self.workers {
            if worker.id == id {
                worker.is_selected = selected;
                let _ = worker.save(&self.ctx);
                return Some(worker);
            }
        }
        None
    }
    pub fn get_by_id(&self, id: Uuid) -> Option<&Worker> {
        self.workers.iter().find(|w| w.id == id)
    }
    pub fn get_workers(&self) -> &Vec<Worker> {
        &self.workers
    }
    pub fn get_workers_selected(&self) -> Vec<&Worker> {
        self.workers
            .iter()
            .filter(|w| w.is_selected)
            .map(|w| w)
            .collect::<Vec<&Worker>>()
    }
}
