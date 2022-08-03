pub mod tokio_run {

    #[tokio::main]
    pub async fn main_run() {
        println!("Here we are in Tokio main");

        let data = get_data().await;

        println!("We GOT the DATA {}", data);
    }

    pub async fn get_data() -> i32 {
        1020
    }
}