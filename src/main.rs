extern crate clap;
extern crate config;
extern crate serde;

use clap::{Arg, App};
use config::*;
use std::path::Path;
use std::process;
use gmv::settings::Settings;
use gmv::git;

fn main() {
    let matches = App::new("grrs")
        .version("0.1.0")
        .author("tiger")
        .about("move sources code")
        .arg(Arg::with_name("FILE")
            .help("Config file path.")
            .empty_values(false)
        )
//        .arg(Arg::with_name("dirls.bat")
//            .short("d")
//            .long("dirls.bat")
//            .help("Exec dirls.bat")
//        )
        .get_matches();

    if matches.is_present("push.bat") {
        println!("dirls.bat is turned on");
        let url = "http://ip:port/demo/demo-rest-gmv.git";
        let name = "demo-rest";
        git::push(name, url);
    } else if let Some(file) = matches.value_of("FILE") {
        println!("Value for file argument: {}", file);
        if Path::new(&file).exists() {
            let mut c = Config::default();
            c.merge(File::new(file, FileFormat::Yaml))
                .unwrap();
            let s: Settings = c.try_into().unwrap();
            println!("settings: {:?}", s);
            git::clone(s);
        } else {
            eprintln!("[gmv Error] No such file or directory.");
            process::exit(1); // 程序错误终止时的标准退出码
        }
    }
}
