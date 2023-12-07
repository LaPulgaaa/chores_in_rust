use clap::Parser;
use chrono::prelude::*;
fn main() {
    println!("Hello, world!");
    add_task();
}
#[derive(Parser,Debug)]
#[command(name="to do list")]
#[command(author,version,about)]
struct Task{
    ///enter title
    title:String,
    ///enter discription of the title
    discription:String,

}
fn add_task(){
    let field=Task::parse();
    println!("To do :");
    println!("task -> {}",field.title);
    println!("details -> {}",field.discription);
}
