use std::{collections::HashMap, time::{SystemTime, SystemTimeError}};
use super::user::User;
use uuid::Uuid;


pub type Token = Uuid;
pub type ConnectionMap = HashMap<Token, Connection>;


pub struct Connection{
    user: Option<User>,
    last_connection_time: SystemTime
}

#[allow(dead_code)]
impl Connection {
    pub fn new(user: Option<User>) -> Connection{
        Connection{
            user, 
            last_connection_time: SystemTime::now()
        }
    }

    pub fn get_user(&self) -> &Option<User>{
        &self.user
    }

    pub fn set_user(&mut self, user: Option<User>){
        self.user = user
    }

    pub fn update_time(&mut self){
        self.last_connection_time = SystemTime::now()
    }

    pub fn get_time(&self) -> SystemTime{
        self.last_connection_time
    }

    pub fn is_time_over(&self, seconds: u64) -> Result<bool, SystemTimeError>{
        let result = self.last_connection_time.elapsed();
        if let Ok(elapsed) = result {
            return Ok(elapsed.as_secs() < seconds)
        }
        Err(result.unwrap_err())
    }
}

pub fn get_token() -> Uuid{
    Uuid::new_v4()
}