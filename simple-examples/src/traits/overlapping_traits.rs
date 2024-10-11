trait UserNameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    userName: String,
    age: u8,
}

impl UserNameWidget for Form {
    fn get(&self) -> String {
        self.userName.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        userName: "Mark".to_owned(),
        age: 12,
    };

    // this will a compiler error, because of multiple get() declaration in traits
    // let userName = form.get();
    
    
    let userName = <Form as UserNameWidget>::get(&form);
    assert!(userName == form.userName);
    println!("userName: {}", userName);

    let age = <Form as AgeWidget>::get(&form);
    assert!(age == form.age);
    println!("age: {}", age);

}