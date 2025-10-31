#[allow(dead_code)]
use std::collections::HashMap;
use std::fmt::{self};

#[derive(Debug)]
pub struct User {
    name: String,
    password: String,
    locked: bool,
}

impl User {
    pub fn new(name: String, password: String) -> Self {
        User {
            name,
            password,
            locked: false
        }
    }
}


#[derive(Debug)]
pub struct Userbase {
    users: HashMap<String, User>
}

impl Userbase {
    pub fn new() -> Self {
        Userbase { 
           users: HashMap::new()
        }
    }

    pub fn add_user(&mut self,user: User) {
        self.users.insert(user.name.clone(), user);
    }

    pub fn get_user(&mut self, name: String) -> Option<&User> {
        self.users.get(&name)
    }

    pub fn authenticate(&mut self, name: String, password: String) -> Result<String, AuthError> {
        if !self.users.contains_key(&name) {
            return Err(AuthError::UserNotFound);
        }
        let user: &mut User = self.users.get_mut(&name).unwrap();
        if user.locked == true {
            return Err(AuthError::UserLocked);
        }
        if !(user.password == password) {
                user.locked = true;
                return Err(AuthError::WrongPassword);
        }
        Ok("user and password correct".to_string())
    }
}

impl fmt::Display for AuthError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserNotFound => write!(f, "user not found"),
            AuthError::WrongPassword => write!(f, "wrong password"),
            AuthError::UserLocked => write!(f, "user locked")

        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    WrongPassword,
    UserLocked,
}
pub trait Export {
    fn export_csv (&self) -> String;

    fn export_json (&self) -> String;
}

impl Export for User {
    fn export_csv(&self) -> String {
        format!("{},{},{}" , &self.name.to_string(), &self.password.to_string(), &self.locked )
    }

    fn export_json(&self) -> String {
        format!("{{ \"name\":\"{}\",\"password\":\"{}\",\"password\":{} }}" , &self.name.to_string(), &self.password.to_string(), &self.locked )
    }
}

impl Export for Userbase {
    fn export_csv(&self) -> String {
        let mut ergebnis: String = "".to_string();
        for (_name, user) in &self.users {
           ergebnis.push_str(&user.export_csv()); 
            ergebnis.push_str("\n");
        }
        ergebnis
    }

    fn export_json(&self) -> String {
        let mut ergebnis: String = "".to_string();
        ergebnis.push('[');
        for (_name, user) in &self.users {
           ergebnis.push_str(&user.export_json()); 
        }
        ergebnis.push(']');
        ergebnis
    }
}