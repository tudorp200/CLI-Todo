# CLI-Todo
A simple and efficient command-line to-do list manager written in Rust Language. I used the crate rusqilte for buildinga database, manage data stora. The key feature of this projects is that commands are done by using SQLite language for, so you can benefit of unreseted to-do list even if you restart you computer.
### Instalation
Install and setup cargo. 
### On Linux and MacOS systems 
```bash
curl https://sh.rustup.rs -sSf | sh
```
### Windows
Download and run rustup-init.exe.

### Linux/Windows/Mac 
For installing the actual tool, this is done as follows:
```bash 
cargo install cli_todo_list_sqlite
```

### Usage
```bash 
# Add a new task
todo add <task_name>

# View all tasks
todo list 

# Remove a task
todo remove <task_id> 

# Mark task as done 
todo done <task_id>

# Remove all the task from the database
todo reset 

# Sorting tasks ascending or descending by the name 
todo sort -asc/-desc

# Listing the remaining tasks 
todo rmn
```

## Example of use:
![Example:](video/output.gif)


## LICENSE
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
