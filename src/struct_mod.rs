
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: i32,
    pub gender: String,
    pub citizen: bool,
    pub reward: Reward,
}

#[derive(Debug)]
pub struct Reward {
    pub name: String,
    pub points: i32,
}

pub fn get_reward() -> Reward {
    let q1 = Reward {
        name: "Login".to_string(),
        points: 23,
    };

    q1
}

pub fn create_activity(rew: Reward) -> User {
    let a1 = User {
        name: "Azhar".to_string(),
        age: 32,
        gender: "Male".to_string(),
        citizen: true,
        reward: rew,
    };

    a1
}

pub fn get_activity() -> User {
    let a = get_reward();
    let u = create_activity(a);

    u
}

pub fn main() {
    get_activity();
}

