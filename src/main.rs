mod civ;
mod helpers;
mod logger;
mod person;
mod world;

use logger::Logger;
use world::World;

const YEARS_TO_GEN: i32 = 10;
const LOG: Logger = Logger;

fn main() {
    let mut w = World::new();

    LOG.legendary_event( &format!("World Name: {} | World Diameter: {}Km \n\nAs fabric of reality unravels, a kaleidoscopic portal emerges, unveiling a pristine world untouched by mortal hands. A symphony of unknown sounds fills the air, resonating with the promise of uncharted mysteries. The stage is set for an odyssey into the unfamiliar. And what wonders lie ahead, one can only ponder.\n\nWhat interesting events will unfold here?", w.name, w.diameter) );

    for i in 1..YEARS_TO_GEN + 1 {
        w.gen_year(i);
    }
}
