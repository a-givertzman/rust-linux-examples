#![allow(non_snake_case)]

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Proggrammer {
    fn language(&self) -> String;
}

trait CompSciStudent: Student + Proggrammer {
    fn gitUserName(&self) -> String;
}

#[derive(Debug)]
struct ExactStudent {
    name: String,
    university: String,
    lang: String,
    git: String,
}
impl CompSciStudent for ExactStudent {
    fn gitUserName(&self) -> String {
        self.git.clone()
    }
} 
impl Proggrammer for ExactStudent {
    fn language(&self) -> String {
        self.lang.clone()
    }
}
impl Student for ExactStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}
impl Person for ExactStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let exStudent = ExactStudent {
        name: "Robert".to_string(),
        university: "Harvard University".to_string(),
        lang: "Go".to_string(),
        git: "robert.coder".to_string(),
    };
    println!("student: {:?}", exStudent);
    println!("student.name(): {:?}", exStudent.name());
    println!("student.university(): {:?}", exStudent.university());
    println!("student.language(): {:?}", exStudent.language());
    println!("student.gitUserName(): {:?}", exStudent.gitUserName());
}