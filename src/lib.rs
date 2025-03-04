use colored::Colorize;
use rusqlite::Connection;
use rusqlite::Result;
use rusqlite::Statement;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyError {}

pub struct Config {
    pub command: String,
    pub content: String,
}

impl Config {
    pub fn config_error() -> Box<dyn Error> {
        let error_msg = "\tUsage:
    1. add <sentence> for adding a task in todo-list
    2. list for listing the tasks in the todo-list
    3. remove <id_task> for removing a task in the todo-list
    4. done <id_task> for marking a task as done
    5. reset for reseting all the tasks in todo-list
    6. sort -asc for sorting tasks ascendent , -desc for sorting tasks descendent by the name
    7. rmn for listing all the remaing tasks that are not done";
        Box::new(MyError(error_msg.to_string()))
    }
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            Err(Self::config_error())
        } else {
            let command1 = args[1].clone();
            //let content1 = args[2].clone();
            let mut content1: String = String::new();
            if args.len() >= 2 {
                for arg in args.iter().skip(2) {
                    content1.push_str(arg);
                    content1.push(' ');
                }
            }
            Ok(Config {
                command: command1,
                content: content1,
            })
        }
    }
    pub fn build_database(path: &str) -> Result<Connection, Box<dyn Error>> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo_list(
            id  INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            status BOOL
            )",
            (),
        )?;
        Ok(conn)
    }
}

pub struct Task {
    id: i32,
    name: String,
    status: bool,
}

impl Task {
    /*fn new(name1: String) -> Task {
        Task {
            id: None,
            name: name1,
            status: false,
        }
    }*/
    pub fn add(name: String, conn: &Connection) -> Result<(), Box<dyn Error>> {
        let id_m: Option<i32> = conn
            .query_row("SELECT MAX(id) FROM todo_list", [], |row| row.get(0))
            .unwrap_or(None);
        let new_id = id_m.unwrap_or(0) + 1;
        conn.execute(
            "INSERT INTO todo_list (id, name, status) VALUES (?1, ?2, ?3)",
            (new_id, &name, false),
        )?;

        Ok(())
    }
    pub fn make_string(id: &i32, name: &str, status: &bool) -> String {
        let mut temp = String::new();
        //let number: i32 = id.parse().unwrap("salut");
        temp.push_str(&id.to_string());
        temp.push('.');
        temp.push_str(name);
        temp.push_str(" : ");
        if *status {
            temp.push_str("done");
        } else {
            temp.push_str("not done");
        }
        temp
    }
    pub fn fetch_tasks(stmt: &mut Statement) -> Result<(), Box<dyn Error>> {
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                status: row.get(2)?,
            })
        })?;
        for task in task_iter {
            let temp = task?;
            let output = Self::make_string(&temp.id, &temp.name, &temp.status);
            if temp.status {
                println!("{}", output.green());
            } else {
                println!("{}", output.red());
            }
        }
        Ok(())
    }

    pub fn list(conn: &Connection) -> Result<(), Box<dyn Error>> {
        let mut stmt = conn.prepare("SELECT id, name, status FROM todo_list")?;
        Self::fetch_tasks(&mut stmt)?;
        Ok(())
    }

    fn search_id(conn: &Connection, id: i32) -> Result<(), Box<dyn Error>> {
        let mut stmt = conn.prepare("SELECT id FROM todo_list")?;
        let id_iter = stmt.query_map([], |row| row.get::<_, i32>(0))?;
        for ids in id_iter {
            let temp = ids?;
            if temp == id {
                return Ok(());
            }
        }

        Err(Box::new(MyError("the id is not in the todo_list!".into())))
    }

    pub fn remove(conn: &Connection, id: i32) -> Result<(), Box<dyn Error>> {
        Self::search_id(conn, id)?;
        conn.execute("DELETE from todo_list WHERE id = ?1", (id,))?;
        Ok(())
    }

    pub fn mark_as_done(conn: &Connection, id: i32) -> Result<(), Box<dyn Error>> {
        Self::search_id(conn, id)?;
        conn.execute("UPDATE todo_list SET status = true WHERE id = ?1", (id,))?;
        Ok(())
    }

    pub fn reset(conn: &Connection) -> Result<(), Box<dyn Error>> {
        conn.execute("DELETE FROM todo_list", ())?;
        Ok(())
    }

    pub fn sort(conn: &Connection, temp: &str) -> Result<(), Box<dyn Error>> {
        let mut stmt;
        if temp == "-asc" {
            stmt = conn.prepare("SELECT id, name, status FROM todo_list ORDER BY name ASC")?;
        } else if temp == "-desc" {
            stmt = conn.prepare("SELECT id, name, status FROM todo_list ORDER BY name DESC")?;
        } else {
            return Err(Box::new(MyError("cannot be sorted!".into())));
        }
        Self::fetch_tasks(&mut stmt)?;

        Ok(())
    }
    pub fn rmn(conn: &Connection) -> Result<(), Box<dyn Error>> {
        let mut stmt =
            conn.prepare("SELECT id, name, status FROM todo_list WHERE status = false")?;
        Self::fetch_tasks(&mut stmt)?;
        Ok(())
    }
}

// IN function run i define the connection to the database
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let conn = Config::build_database("./my_db.db3")?;
    //println!("A intrat si aici!");
    match config.command.as_str() {
        "add" => {
            Task::add(config.content, &conn)?;
        }
        "list" => {
            Task::list(&conn)?;
        }
        "remove" => {
            let id: i32 = config.content.trim().parse()?;
            Task::remove(&conn, id)?;
        }
        //"edit" =>
        "done" => {
            let id: i32 = config.content.trim().parse()?;
            Task::mark_as_done(&conn, id)?;
        }
        "reset" => {
            Task::reset(&conn)?;
        }
        "sort" => {
            let temp = config.content.trim();
            Task::sort(&conn, temp)?;
        }
        "rmn" => {
            Task::rmn(&conn)?;
        }
        _ => {
            return Err(Config::config_error());
        }
    }
    Ok(())
}
