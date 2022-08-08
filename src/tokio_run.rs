pub mod tokio_run {

    #[tokio::main]
    pub async fn main_run() {
        println!("Here we are in Tokio main");

        let data = get_data().await;

        println!("We GOT the DATA {}", data);
    }

    pub async fn get_data() -> i32 {
        let more = new_data().await;
        1020 + more
    }

    pub async fn new_data() -> i32 {
        200
    }
}
