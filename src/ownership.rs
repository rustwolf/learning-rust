pub mod ownership {
    struct Item {
        what: String,
        present: bool,
    }

    struct Hands {
        left: Item,
        right: Item,
    }

    pub fn main() {
        let mut hands = Hands {
            left: Item {
                what: String::from("Apple"),
                present: true,
            },
            right: Item {
                what: String::from("Orange"),
                present: true,
            },
        };

        fun_name(&hands);

        let air = hands.left;
        hands.left = hands.right;
        hands.right = air;

        fun_name(&hands);
    }

    fn fun_name(hands: &Hands) {
        if hands.left.present {
            println!("Left hand is holding something");
        } else {
            println!("Left hand is not holding anything");
        }
        if hands.right.present {
            println!("Right hand is holding something");
        } else {
            println!("Right hand is not holding anything");
        }
    }
}
