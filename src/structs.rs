pub mod structs {

    #[derive(Debug)]
    pub struct User {
        name: String,
        email: String,
        is_active: bool,
        sign_in_count: u32,
    }

    #[derive(Default, Debug)]
    pub struct Vehicle {
        color : String,
        wheels_count : u32,
        speed_limit : u32,
        is_automatic : bool
    }

    pub fn create_user() -> User {
        let azhar = User {
            name: String::from("Azhar uddin"),
            email: String::from("az.az@gmail.com"),
            is_active: true,
            sign_in_count: 20,
        };

        let a = Vehicle{
            ..Default::default()
        };

        println!("{:?}", a);

        azhar
    }
}
