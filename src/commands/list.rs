use prettytable::{
    format::{self, TableFormat},
    row, Table,
};

use crate::config::Config;

pub fn execute() {
    let config = Config::load();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(row![bc=>"Name", "Path"]);
    for java_home in config.java_homes {
        if let Ok(path) = java_home.path.canonicalize() {
            table.add_row(row![java_home.name, path.to_string_lossy().to_string()]);
        }
    }

    table.printstd();

    if let Some(global) = config.global_java {
        println!("\nThe global java version is: {}", global)
    }

    println!("\nThe current java version is:TODO") //TODO
}
