pub mod userdata;
pub mod generate_body;
pub mod httpreq;

use clap::{Parser, Subcommand, Args};
use chrono::{Datelike, Local};

// basic arguments
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Cli {
    // subcommands
    // - regist
    // - get
    #[clap(subcommand)]
    pub action: Action,
}

// #[derive(Args)]
// pub struct RegistArgs { 
//     // student name
//     #[clap(short, long)]
//     pub name: String,

//     // student number
//     #[clap(short, long)]
//     pub stnum: String,
// }

#[derive(Args)]
pub struct GetArgs { 
    #[clap(short, long, default_value_t = Local::today().year())]
    pub year: i32,

    #[clap(short, long, default_value_t = Local::today().month())]
    pub month: u32,
    
    #[clap(short, long, default_value_t = Local::today().day())]
    pub day: u32,
}

#[derive(Subcommand)]
pub enum Action {
    // Regist(RegistArgs),
    Regist,
    Get(GetArgs),
}
