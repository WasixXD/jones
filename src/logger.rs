extern crate colored;


use colored::*;




pub struct Logger;



impl Logger {


    pub fn normal_event(&self, text: &str) {
        println!("{}", text.bold().italic());
    }


    pub fn epic_event(&self, text: &str) {
        println!("{}\n", text.bold().purple().underline());
    }


        

}
