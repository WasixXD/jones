use crate::person::Person;
use crate::helpers::new_name;

use rand::Rng;



#[derive(Debug, Clone)]
pub struct Civ {
    pub name: String,
    pub foundation: i32,
    t_existence: i32,
    

    pub main_religion: String,
    villagers: Vec<Person>,
    pub ruler: Person,


    birth_rate: f32,
    birth_multiplicator: f32,




}



impl Civ {
    pub fn new(f: i32) -> Self {
        let mut r = rand::thread_rng();
        let founder = new_name();
        Civ {
            name: new_name(),
            foundation: f,
            t_existence: 0,

            main_religion: "asdf".to_string(),
            villagers: vec!(Person{ name: founder.clone() }),
            ruler: Person{ name: founder.clone() }, 

            birth_rate: r.gen_range(0.01..0.05),
            birth_multiplicator: r.gen_range(0.5..2.5),
        }

    }



    pub fn gen_pop(&self) {
        let population = self.villagers.len() as i32;
        
        let result = (f32::powf(-self.birth_rate, self.t_existence as f32) + self.birth_multiplicator * self.t_existence as f32) as i32;

        println!("{} - {}", self.name, result);

        
    }


    pub fn add_year(&mut self) {
        self.t_existence+= 1;
    } 
}



