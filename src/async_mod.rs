pub mod async_mod {
    
    pub async fn run_async() -> i32 {
        10
    }

    pub async fn main() -> i32 {
        // let data = run_async().await;
        let mut a = 30;

        let b = async {
            a = run_async().await;
            println!("We are in Async {}", a);
        };        
        200
    }
}