use clap::Parser;
// use chrono::prelude::*;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

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
    
    //reading the existing task
    let mut task_file=File::open("task.txt").unwrap();

    let mut old_task=String::new();

    task_file.read_to_string(&mut old_task).unwrap();

    let list:Note=serde_json::from_str(&old_task).expect("error reading the task file");

    println!("Task :{} ,{}",list.task,list.describe);

    //serializing the struct
    let unit_task=serde_json::to_string(&unit_task).unwrap();

    //writing file 

    // let mut file=File::create("task.txt").expect("error creating file");
    // file.write(unit_task.as_bytes()).expect("error writing the file");


    //appending a file
    let mut file=OpenOptions::new().append(true).open("task.txt").expect("error opening the file");
    file.write(&unit_task.as_bytes()).expect("error appendint to file");


}


fn welcome_msg(){
    let mut file=File::open("welcome.txt").expect("could not open the file!!");
    let mut welcome=String::new();

    file.read_to_string(&mut welcome).expect("could not read the file to content string");
    println!("{}",welcome);


    
}

