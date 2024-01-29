use crate::person::Person;
use crate::helpers::new_name;



#[derive(Debug)]
pub struct Civ {
    pub name: String,
    pub foundation: i32,
    t_existence: i32,
    

    pub main_religion: String,
    villagers: Vec<Person>,
    pub ruler: Person,


}



impl Civ {
    pub fn new(f: i32) -> Self {
        let founder = new_name();
        Civ {
            name: new_name(),
            foundation: f,
            t_existence: 0,

            main_religion: "asdf".to_string(),
            villagers: vec!(Person{ name: founder.clone() }),
            ruler: Person{ name: founder.clone() }, 
        }

    }
}



