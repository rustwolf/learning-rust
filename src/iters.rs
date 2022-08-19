pub mod iters {
    pub fn main() {
        let names = vec!["Azhar K", "Rajdeep B", "Sheetal P", "Ganga Y"];

        // let vals = names.iter().map(|a| println!("{}", a));

        for (n, i) in names.iter().enumerate() {
            println!("{} at {}", n, i);
        }

        // names.iter().map(|name| format!("{}{}", name, " is very good boy/girl")).collect();
    }
}
