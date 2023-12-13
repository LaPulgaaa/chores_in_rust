use clap::Parser;
// use chrono::prelude::*;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize,Serialize};

use std::io::prelude::*;

fn main() {
    
    welcome_msg();
    add_task();
    write_task();
    
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

fn add_task()->Vec<String>{
    let field=Task::parse();
   
    //struct for one task
    

    let add_task=Todo{
        name:field.name,
        task:field.title,
        describe:field.discription,
        status:Status::Uncomplete(String::from("Not Completed"))
    };

    println!("name:=>{}",add_task.name);
    println!("task:=>{}",add_task.task);
    println!("discription:=>{}",add_task.describe);
    println!("{:?}",add_task.status);

    let mut send_data:Vec<String>=Vec::new();
    send_data.push(add_task.name);
    send_data.push(add_task.task);
    send_data.push(add_task.describe);

    return send_data


}


fn welcome_msg(){
    let mut file=File::open("welcome.txt").expect("could not open the file!!");
    let mut welcome=String::new();

    file.read_to_string(&mut welcome).expect("could not read the file to content string");
    println!("{}",welcome);
}

fn write_task(){
    
    

    let mut task_file=File::create("task.txt").expect("could not create a file");
    // task_file.write_all(name.as_bytes()).expect("could not write the task to the file!!");

    
    let j_file=serde_json::to_string(&write).unwrap();

    //write a serialized json object 
    task_file.write_all(j_file.as_bytes()).expect("could not write json string to file");

}