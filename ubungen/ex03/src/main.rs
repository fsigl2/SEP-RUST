#[allow(dead_code)]
mod accounts;
use accounts::{User, Userbase};

use crate::accounts::Export;

fn main() {
    let flo: User = User::new("flo".to_string() , "flo123".to_string() );
    let jost: User = User::new("jost".to_string() , "jost123".to_string() );
    let maxi: User = User::new("maxi".to_string() , "maxi123".to_string() );

    let mut users: Userbase = Userbase::new();

    users.add_user(flo);
    users.add_user(jost);
    users.add_user(maxi);

//    println!("{:?}", users.get_user("flo".to_string()));
//    println!("{:?}", users.get_user("jost".to_string()));
//    println!("{:?}", users.get_user("maxi".to_string()));
    println!("{}", users.export_csv());
    println!("{}", users.export_json());
    println!("{:?}", users.authenticate("klo".to_string(), "test123".to_string()));
}