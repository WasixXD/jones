use rand::Rng; 

use crate::logger::Logger;
use crate::civ::Civ;
use crate::helpers::new_name;

#[derive(Debug)]
pub struct World {
    name: String, 
    cities: Vec<i32>,
    diameter: i32,
}  


const LOG: Logger = Logger;




impl World {

    pub fn gen_year(&self, year: i32) {
        let mut rng = rand::thread_rng();

        let civ_spawn_rate = 0.08f32;

        let r_number: f32 = rng.gen();

        if (r_number / 10f32) >= civ_spawn_rate {
            let civ = Civ::new(year);
            

            LOG.epic_event( &format!("{:?} has founded a new civilization! He/She named it {}", civ.ruler.name, civ.name) );
        }
        
    }



    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        World{name: new_name(), cities: vec!(0), diameter: rng.gen_range(800..12742*2)}

    }
}
