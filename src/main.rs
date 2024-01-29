
mod world;
mod logger;
mod civ;
mod helpers;
mod person;


use world::World;


const YEARS_TO_GEN: i32 = 10;



fn main() {
    let w = World::new();

    for i in 0..YEARS_TO_GEN {
        
        w.gen_year(i);


    }

}
