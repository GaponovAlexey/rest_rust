use std::{ io::{ self, Write, BufRead, BufReader }, path::Path, fs::File };
use serde::{ Deserialize, Serialize };
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
enum Job {
    Easy,
    Medium,
    Hard,
}

impl Job {
    fn to_string(&self) -> String {
        match self {
            Job::Easy => "Easy way".to_owned(),
            Job::Medium => "Medium way".to_owned(),
            Job::Hard => "Hard way".to_owned(),
        }
    }
}
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    job: Job,
    time: DateTime<Local>,
}

impl User {
    fn new(name: String, age: u8, job: Job) -> Self {
        Self { name, age, job, time: Local::now() }
    }

    fn get_input(query: &str) -> io::Result<String> {
        println!("{}", query);
        io::stdout().flush()?;

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        Ok(buf.trim().to_owned())
    }
    fn print(&self) {
        println!(
            "*****\n{}\n{}\n{}\n{}\n*****",
            self.name,
            self.age,
            self.job.to_string(),
            self.time
        )
    }

    fn new_from_console() -> Self {
        let name = Self::get_input("your name").expect("err");

        let mut age: Option<u8> = None;
        while age.is_none() {
            match Self::get_input("your age").expect("err").parse::<u8>() {
                Ok(v) => {
                    age = Some(v);
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }

        let job = match Self::get_input("your Job").expect("err").as_str() {
            "1" => Job::Easy,
            "2" => Job::Medium,
            "3" => Job::Hard,
            _ => {
                println!("You work easy");
                Job::Easy
            }
        };
        Self::new(name, age.expect("err"), job)
    }
}

struct Manager {
    user: Vec<User>,
}

impl Manager {
    fn new() -> Self {
        Self { user: Vec::new() }
    }

    fn print_user(&mut self) {
        for user in &self.user {
            user.print();
        }
    }
    fn add_user(&mut self, user: User) {
        self.user.push(user)
    }
    fn find_user(&mut self, name: &str) -> Option<usize> {
        self.user.iter().position(|e| e.name == name)
    }
    fn remove_user(&mut self, name: &str) -> Result<String, String> {
        if let Some(e) = self.find_user(&name) {
            self.user.remove(e);
            Ok("removed".to_string())
        } else {
            Err("err".to_string())
        }
    }
    fn edit_user(&mut self, name: &str, update_task: User) -> Result<String, String> {
        if let Some(i) = self.find_user(&name) {
            match self.user.get_mut(i) {
                Some(user) => {
                    user.name = update_task.name;
                    user.age = update_task.age;
                    user.job = update_task.job;
                    Ok("update".to_string())
                }
                None => todo!(),
            }
        } else {
            Err("err".to_string())
        }
    }
    fn store_to_file(&self, file_name: &str) -> Result<String, String> {
        if !Path::new(file_name).exists() {
            let file = match File::create(file_name) {
                Ok(f) => f,
                Err(e) => {
                    return Err("err".to_string());
                }
            };
            match serde_json::to_writer(&file, &self.user) {
                Ok(_) => Ok("ok".to_owned()),
                Err(_) => Err("ok".to_owned()),
            }
        } else {
            return Err("err".to_string());
        }
    }
    fn read_to_file(&mut self, file_name: &str) -> Result<String, String> {
        if Path::new(file_name).exists() {
            let file = match File::open(file_name) {
                Ok(f) => f,
                Err(e) => {
                    return Err("err".to_string());
                }
            };
            let reader = BufReader::new(file);
            self.user = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(e) => {
                    return Err(format!("err{}", e));
                }
            };
            Ok("Successfully".to_owned())
        } else {
            return Err("err".to_string());
        }
    }
}

struct Console {
    manager: Manager,
    from_console: Vec<String>,
}

impl Console {
    fn new() -> Self {
        Self {
            manager: Manager::new(),
            from_console: vec![
                "Add Task".to_owned(),
                "Print task".to_owned(),
                "Find task".to_owned(),
                "Remove task".to_owned(),
                "Edit task".to_owned(),
                "Store task to file".to_owned(),
                "Read task from file".to_owned()
            ],
        }
    }
    fn print_menu(&self) {
        for (i, e) in self.from_console.iter().enumerate() {
            println!("{}:{}", i + 1, e);
        }
    }
    fn console_command(&mut self) {
        Self::print_menu(self);
        match User::get_input("Enter command") {
            Ok(res) => {
                match res.as_str() {
                    "1" => self.manager.add_user(User::new_from_console()),
                    "2" => self.manager.print_user(),
                    "3" => {
                        let name = User::get_input("name to find").expect("err");
                        if let Some(i) = self.manager.find_user(name.as_str()) {
                            self.manager.user.get(i).expect("err").print()
                        } else {
                            eprint!("err")
                        }
                    }
                    "4" => {
                        let name = User::get_input("name to find").expect("err");
                        match self.manager.remove_user(name.as_str()) {
                            Ok(e) => println!("{:?}", e),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    "5" => {
                        let name = User::get_input("name to find").expect("err");
                        match self.manager.edit_user(&name, User::new_from_console()) {
                            Ok(e) => println!("{:?}", e),
                            Err(e) => eprint!("{}", e),
                        }
                    }
                    "6" => {
                        let file_name = User::get_input("name to find").expect("err");
                        self.manager.store_to_file(file_name.as_str()).expect("err");
                    }
                    "7" => {
                        let file_name = User::get_input("name to find").expect("err");
                        self.manager.read_to_file(file_name.as_str()).expect("err");
                    }
                    _ => { println!("not command") }
                }
            }
            Err(_) => eprint!("err"),
        }
    }
}

fn main() {
    let mut app = Console::new();

    loop {
        app.console_command();
        println!("");
    }
}
