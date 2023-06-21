use serde::{Serialize, Deserialize};
use std::{sync::{Arc}, collections::HashMap};
use parking_lot::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Clone)]
pub struct AppState {
    pub user_cache: Arc<RwLock<HashMap<i32, User>>>
}

pub fn prepare_users() -> HashMap<i32, User>{
    let mut users = HashMap::<i32, User>::new();
    {
        for i in 1..10 {
            users.insert(i, User{ 
                id: i, 
                name: format!("user {}", i), 
                email: format!("user{}@gmail.com", i), 
                age: 20, 
            });
        }
    }
    users
}