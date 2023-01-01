mod old {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};
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
}
