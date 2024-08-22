pub mod memory() {
    pub fn my_fn() {
        let fname: String = String::from("Abhishek");

        print_name(&fname);

        println!("Name has been printed {}", fname);
    }

    pub fn print_name(name: &str) {
        println!("Nmae is {}", name);
    }
}