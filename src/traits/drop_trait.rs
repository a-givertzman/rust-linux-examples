#![allow(non_snake_case)]

struct Dropable {
    name: &'static str,
}

impl Drop for Dropable {
    fn drop(&mut self) {
        println!("{:?}: goodby!", self.name);
    }
}

fn main() {
    let _root1 = Dropable { name: "root 1" };
    let root2 = Dropable { name: "root 2" };
    '_blokA: {
        let _blockA = Dropable { name: "Block A" };
        '_blockB: {
            let _blockB1 = Dropable { name: "Block B1" };
            let _blockB2 = Dropable { name: "Block B2" };
            println!("exit blockB...");
        }
        println!("exit blockB - ok");
        println!("exit blockA...");
    }
    println!("exit blockA - ok");
    println!("dropping root1...");
    drop(root2);
    println!("dropping root1 - ok");
    println!("exit root...")
}