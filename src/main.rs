use std::io::{ self, Write };

use chrono::{ DateTime, Local };

enum Job {
    SurvivalJob,
    TechSupport,
    FrontEnd,
    BackEnd,
}
impl Job {
    fn to_string(&self) -> &str {
        match self {
            Job::SurvivalJob => "low money",
            Job::TechSupport => "low money more SurvivalJob",
            Job::FrontEnd => "Good price",
            Job::BackEnd => "Perfect price",
        }
    }
}
struct PersonalInfo {
    name: String,
    age: u8,
    country: String,
}

impl PersonalInfo {
    fn new(name: String, age: u8, country: String) -> Self {
        Self { name, age, country }
    }

    fn new_from_console() -> Self {
        let name = Works::get_input("Enter your name").expect("err name input");
        let country = Works::get_input("Enter your country").expect("err country input");

        let mut age = None;
        while age.is_none() {
            match Works::get_input("enter age").expect("err").parse::<u8>() {
                Ok(v) => {
                    age = Some(v);
                }
                Err(_) => println!("Please Enter Number"),
            }
        }

        Self::new(name, age.expect("err"), country)
    }
}

struct JobInfo {
    job: Job,
    time: DateTime<Local>,
}

impl JobInfo {
    fn new(job: Job, time: DateTime<Local>) -> Self {
        Self { job, time }
    }
    fn print_job_list() {
        println!("1: Sur\n2: Support\n3: Frontend\n4: Backend");
    }
    fn new_from_console() -> Self {
        Self::print_job_list();
        Self::new(
            match
                Works::get_input("Enter your specialization").expect("err").to_lowercase().as_str()
            {
                "1" => Job::SurvivalJob,
                "2" => Job::TechSupport,
                "3" => Job::FrontEnd,
                "4" => Job::BackEnd,
                _ => {
                    println!("your work SurvivalJob");
                    Job::SurvivalJob
                }
            },
            Local::now()
        )
    }
}
struct Works {
    personal_info: PersonalInfo,
    job_info: JobInfo,
}

impl Works {
    fn new(personal_info: PersonalInfo, job_info: JobInfo) -> Self {
        Self { personal_info, job_info }
    }

    fn get_input(query: &str) -> io::Result<String> {
        print!("{:?}", query);
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn new_from_console() -> Self {
        Self::new(PersonalInfo::new_from_console(), JobInfo::new_from_console())
    }

    fn print_details(&self) {
        println!(
            "\n\nName: {}\nAge: {}\nCountry: {}\nJob: {}\nTime: {}",
            self.personal_info.name,
            self.personal_info.age,
            self.personal_info.country,
            self.job_info.job.to_string(),
            self.job_info.time
        );
    }
}

struct WorksManager {
    works: Vec<Works>,
}

impl WorksManager {
    fn new() -> Self {
        Self { works: Vec::new() }
    }

    fn print_job(&self) {
        for job in &self.works {
            println!("from Manager");
            job.print_details();
        }
    }
    fn add_task(&mut self, name: Works) {
        self.works.push(name)
    }
}

struct ConsoleManager {
    work_manager: WorksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            work_manager: WorksManager::new(),
            menu_options: vec!["add work".to_owned(), "print works".to_owned()],
        }
    }

    fn print_menu(&self) {
        for (i, m) in self.menu_options.iter().enumerate() {
            println!("{:?}{:?}", i + 1, m);
        }
    }

    fn process_command(&mut self) {
        match Works::get_input("Enter comand index") {
            Ok(inp) => {
                match inp.as_str() {
                    "1" => { self.work_manager.add_task(Works::new_from_console()) }
                    "2" => { self.work_manager.print_job() }
                    _ => { println!("I don't understand") }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();
    loop {
        manager.process_command();
        println!("");
    }
}
