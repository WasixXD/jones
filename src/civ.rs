use crate::person::Person;
use crate::helpers::new_name;
use crate::world::World;


use rand::Rng;
use std::collections::HashMap;


const BIRTH_RATE_GROWTH: f32 = 0.015f32;
const BIRTH_RATE_SHRINK: f32 =  0.02f32;




#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CivStates {
    War,
    Peace,
    Research,

}

pub struct Declaration {
    pub civ_name: String,
    pub enemy_name: Option<String>,
    pub typeOf : CivStates,

    pub why: String
}



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


    states: Vec<CivStates>,
    states_prob: HashMap<CivStates, HashMap<CivStates, f32>>,
    pub c_state: CivStates,




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

            states: vec!(
                CivStates::War,
                CivStates::Peace,
                CivStates::Research),

            // TODO: Ugly
            states_prob: HashMap::from([
                    (CivStates::Peace, HashMap::from([
                                                    (CivStates::Peace, 0.7),
                                                    (CivStates::War, 0.3),
                                                    (CivStates::Research, 0.4),
                    ])),

                    (CivStates::War, HashMap::from([
                                                   (CivStates::Peace, 0.5),
                                                   (CivStates::War, 0.5),
                                                   (CivStates::Research, 0.2),
                    ])),

                    (CivStates::Research, HashMap::from([
                                                        (CivStates::Peace, 0.4),
                                                        (CivStates::War, 0.2),
                                                        (CivStates::Research, 0.7),
                    ])),
                ]),


            c_state: CivStates::Peace,
        }

    }



    pub fn gen_pop(&mut self) {
        let population = self.villagers.len() as i32;
        
        let result = (f32::powf(-self.birth_rate, self.t_existence as f32) + self.birth_multiplicator * self.t_existence as f32) as i32;

        let pop_gen = (population - result).abs();

        for i in 0..pop_gen {
            let human = Person {
                name: new_name()
            };

            self.villagers.push(human);
        }


         

        
    }


    pub fn next_state(&mut self) {

        let mut r = rand::thread_rng();
        

        // Markov Chain
        let randVal: f32 =r.gen();
        let transitionProb= self.states_prob.get(&self.c_state).unwrap();
        let mut cumulativeProb = 0f32;


        for (state, prob) in transitionProb {
            cumulativeProb += prob;


            if randVal <= cumulativeProb {
                self.c_state = state.clone();
            }
        }



    }


    pub fn act(&mut self, civs: Vec<Civ> ) -> Declaration {

        match self.c_state {
            CivStates::Peace => {

                // in peace the birth_rate grow
                self.birth_rate += BIRTH_RATE_GROWTH;

                Declaration {civ_name: self.name.clone(), enemy_name: None, typeOf: self.c_state.clone(), why: "não sei".to_string() }

            },

            CivStates::War => {
                let mut r = rand::thread_rng();

                // in war the birth_rate shrinks
                self.birth_rate -= BIRTH_RATE_SHRINK;
                

                println!("WAR");

                // search for a enemy
                //
                let mut enemy: Civ = self.clone();

                if civs.len() > 1 {
                    while enemy.name == self.name {
                        let n = r.gen_range(0..civs.len());

                        enemy = civs[n].clone();
                    }

                }

                Declaration {civ_name: self.name.clone(), enemy_name: Some(enemy.name), typeOf: self.c_state.clone(), why: "guerra".to_string() }
                
            },
            CivStates::Research => {


                Declaration {civ_name: self.name.clone(), enemy_name: None, typeOf: self.c_state.clone(), why: "Pesquisa".to_string() }
            },
        }
    }


    pub fn add_year(&mut self) {
        self.t_existence+= 1;
    } 
}



