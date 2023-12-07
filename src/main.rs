use clap::Parser;
use chrono::prelude::*;
fn main() {
    println!("Hello, world!");
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
fn add_task(){
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
    println!("{:?}",add_task.status)



}
