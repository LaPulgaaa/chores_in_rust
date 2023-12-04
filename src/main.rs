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
    #[arg(long)]
    title:String,
    #[arg(long)]
    discription:String,
    #[arg(long)]
    deadline:DateTime<Local>
}
fn add_task(){
    let field=Task::parse();
    println!("{}",field.title);
}
