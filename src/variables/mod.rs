static mut QU: Vec<String> = vec![];

pub fn declare_variables() {
    let name = "Azhaer";
    let roll_no = 21;
    println!("{} has roll no {}", name, roll_no);
}

pub fn process_record(str: String) {
    unsafe {
        QU.push(str);

        if QU.len() >= 10 {
            for i in 0..9 {
                print!("{}", QU[i]);
            }
            QU = vec![];
        }
    }
}
