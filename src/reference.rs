pub mod refereneces {
    pub fn main() {
        println!("WE are in references area");

        let num = 10;
        let ref_num = &num;

        println!("{}", *ref_num);
    }
}