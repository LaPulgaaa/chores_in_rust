use clap::Parser;
// use chrono::prelude::*;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize,Serialize};
use std::io::prelude::*;

fn main() {
    
    welcome_msg();
    add_task();
    
}
#[derive(Debug)]
enum Status {
    Complete(String),
    Uncomplete(String)
}
#[derive(Parser,Debug)]
#[command(name="to do list")]
#[command(author,version,about)]
struct Task{

    #[arg(long,short,default_value_t =String::from("Varun Singh"))]
    name:String,

    #[arg(long,short)]
    title:String,

    #[arg(long,short)]
    discription:String,

}
#[derive(Serialize,Deserialize,Debug)]
struct Note{
    name:String,
    task:String,
    describe:String
}
struct Todo{
    name:String,
    task:String,
    describe:String,
    status:Status
}

fn add_task(){
    let field=Task::parse();
   
    //struct for one task
    

    let add_task=Todo{
        name:field.name,
        task:field.title,
        describe:field.discription,
        status:Status::Uncomplete(String::from("Not Completed"))
    };

    let unit_task=Note{
        name:add_task.name,
        task:add_task.task,
        describe:add_task.describe
    };
    

    //serializing the struct
    let unit_task=serde_json::to_string(&unit_task).unwrap();

    //writing file 

    let mut file=File::create("task.txt").expect("error creating file");
    file.write(unit_task.as_bytes()).expect("error writing the file");

    


}


fn welcome_msg(){
    let mut file=File::open("welcome.txt").expect("could not open the file!!");
    let mut welcome=String::new();

    file.read_to_string(&mut welcome).expect("could not read the file to content string");
    println!("{}",welcome);
}

