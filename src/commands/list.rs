use crate::config::Config;

pub fn execute() {
    let config = Config::load();

    println!("Java homes registered in jvenv:");
    for java_home in config.java_homes {
        if let Ok(path) = java_home.path.canonicalize() {
            println!("- {}", path.to_string_lossy())
        }
    }
}
