use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use storaget::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub employers: Vec<Employer>,
    pub employer_counter: u32,
    pub worker_counter: u32,
    pub workers: Vec<Worker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Employer {
    pub id: u32,
    pub name: String,
    pub taxnumber: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Worker {
    pub id: u32,
    pub name: String,
    pub taj: String,
    pub taxnumber: String,
    pub mothersname: String,
    pub birthdate: NaiveDate,
    pub birthplace: String,
    pub zip: u32,
    pub city: String,
    pub street: String,
    pub is_selected: bool,
}

impl Worker {
    pub fn new(
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
    ) -> Self {
        Worker {
            id,
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
}

impl Data {
    pub fn add_new_worker(&mut self, worker: Worker) -> Result<u32, String> {
        self.worker_counter += 1;
        let new_worker = Worker {
            id: self.worker_counter,
            ..worker
        };
        self.workers.push(new_worker);
        Ok(self.worker_counter)
    }
    pub fn update_worker(&mut self, new_worker: Worker) -> Result<&Worker, String> {
        for worker in &mut self.workers {
            if worker.id == new_worker.id {
                *worker = new_worker;
                return Ok(worker);
            }
        }
        Err(format!("Worker not found by ID"))
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
    pub fn set_worker_selected_by_id(&mut self, id: u32, selected: bool) -> Option<&mut Worker> {
        for worker in &mut self.workers {
            if worker.id == id {
                worker.is_selected = selected;
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
    pub fn remove_worker_by_id(&mut self, id: u32) -> Option<Worker> {
        if let Some(pos) = self.workers.iter().position(|x| x.id == id) {
            return Some(self.workers.remove(pos));
        }
        None
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
