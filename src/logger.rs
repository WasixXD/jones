extern crate colored;


use colored::*;




pub struct Logger;



impl Logger {


    pub fn normal_event(&self, text: &str) {
        println!("{}\n\n", text.bold().italic());
    }


    pub fn epic_event(&self, text: &str) {
        println!("{}\n\n", text.bold().purple().underline());
    }



    pub fn legendary_event(&self, text: &str) {
        println!("{}\n\n", text.bold().yellow());
    }

        

}
