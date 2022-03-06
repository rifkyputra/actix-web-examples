use std::sync::Mutex;

use actix_web::{
    web::{self, Data, Path},
    Route,
};
use chrono::Utc;

pub struct VisitBoard {
    persons: Mutex<Vec<Person>>,
}

#[derive(PartialEq, Clone)]
struct Person {
    name: String,
    date: String,
    local_date: String,
}

impl Person {
    fn named(name: &str) -> Person {
        Person {
            date: String::from("Never"),
            name: String::from(name),
            local_date: String::from("Never"),
        }
    }
}

impl VisitBoard {
    pub fn init_state() -> Data<VisitBoard> {
        return web::Data::new(VisitBoard {
            persons: Mutex::new(Vec::new()),
        });
    }
}

pub fn route() -> Route {
    web::get().to(index)
}

async fn index(data: web::Data<VisitBoard>, path: Path<String>) -> String {
    let name_args = path.into_inner();

    println!("{name_args}");

    let mut person_mutex = data.persons.lock().unwrap();

    let mut person = &Person::named(&name_args);

    let now = Utc::now().to_string();

    let position = person_mutex.iter().position(|p| p.name == name_args);

    if position.is_some() {
        person = person_mutex.iter().nth(position.unwrap()).unwrap();
    }

    let name = &person.name.clone();
    let date = &person.date;
    let local_date = &person.local_date;

    let text: String = format!("hello  {name} \n last visit : {date} \n local date: {local_date}");

    if position.is_some() {
        let new_vec = person_mutex
            .iter()
            .map(|e| {
                if e.name == name_args {
                    Person {
                        name: e.name.clone(),
                        date: now.clone(),
                        local_date: now.clone(),
                    }
                } else {
                    e.to_owned()
                }
            })
            .collect::<Vec<Person>>();

        *person_mutex = new_vec;
    }

    if position.is_none() {
        let mut new_vec = person_mutex
            .iter()
            .map(|e| e.to_owned())
            .collect::<Vec<Person>>();

        new_vec.push(Person {
            name: name_args.clone(),
            date: now.clone(),
            local_date: now.clone(),
        });

        *person_mutex = new_vec;
    }

    return text;
}
