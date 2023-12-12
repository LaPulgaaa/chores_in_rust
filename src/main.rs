use clap::Parser;
// use chrono::prelude::*;
use std::fs::File;

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
fn add_task()->Vec<String>{
    let field=Task::parse();
   
    //struct for one task
    struct Todo{
        name:String,
        task:String,
        describe:String,
        status:Status
    }

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
    let mut input_data=add_task();
    let name:&String=&input_data[0];

    let mut task_file=File::create("task.txt").expect("could not create a file");
    task_file.write_all(b"do this by sunday").expect("could not write the task to the file!!");

}