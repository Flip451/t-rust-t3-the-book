use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please write command.");

        let mut command = String::new();

        io::stdin() //  stdin 関数は、std::io::Stdin オブジェクトを返し、この型は、ターミナルの標準入力へのハンドルを表す
            .read_line(&mut command) // ユーザから入力を受け付け
            .expect("Failed to read line");

        let command: Vec<&str> = command.split_whitespace().collect();
        
        let length = command.len();

        if length < 4 {
            println!("Given command is too short!");
            continue;
        } else if length < 4 {
            println!("Given command is too long!");
            continue;
        }

        let verb = command[0].trim().to_string();
        match &verb[..] {
            "Add" => {
                println!("Good verb!");
            },
            _ => {
                println!("Bad verb..");
                continue;
            }
        }

        let particle = command[2].trim().to_string();
        match &particle[..] {
            "to" => {
                println!("Good particle!");
            },
            _ => {
                println!("Bad particle..");
                continue;
            }
        }

        let name = command[1].trim().to_string();
        let department = command[3].trim().to_string();

        let department = employees.entry(department).or_insert(vec![]);
        department.push(name);

        println!("{:?}", employees);
    }
}
