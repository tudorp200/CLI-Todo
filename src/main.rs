use cli_todo_list_sqlite::run;
use cli_todo_list_sqlite::Config;
use std::env;
use std::error::Error;
use std::process;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);
    if let Err(e) = config {
        println!("{e}");
    } else {
        let temp = config.unwrap();
        if let Err(e) = run(temp) {
            println!("{e}");
            process::exit(1);
        }
    }
    Ok(())
}
