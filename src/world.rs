use rand::Rng; 

use crate::logger::Logger;
use crate::civ::Civ;
use crate::helpers::new_name;

#[derive(Debug)]
pub struct World {
    pub name: String, 
    cities: Vec<Civ>,
    pub diameter: i32,
}  


const LOG: Logger = Logger;




impl World {

    pub fn gen_year(&mut self, year: i32) {
        LOG.normal_event( &format!("{year}:") );
        let mut rng = rand::thread_rng();

        let civ_spawn_rate = 0.08f32;

        let r_number: f32 = rng.gen();

        if (r_number / 10f32) >= civ_spawn_rate {
            let c = self.add_civ(year);
            LOG.epic_event( &format!("{:?} has founded a new civilization! He/She named it {}", c.ruler.name, c.name) );
        }


        for c in &mut self.cities {
            c.add_year();
            c.gen_pop();

        }
        
    }

    pub fn add_civ(&mut self, year: i32) -> Civ {
        
        let civ = Civ::new(year);
        self.cities.push(civ.clone());
        civ
    }


    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        World{name: new_name(), cities: vec!(), diameter: rng.gen_range(800..12742*2)}

    }
}
