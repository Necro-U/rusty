struct User {
    name: String,
    username: String,
    e_mail: String,
    sign_in: bool,
    count: i32,
}
fn main() {
    let mut me = User {
        name: String::from("Ubeydullah"),
        username: String::from("hikmetli"),
        e_mail: String::from("hikmetli@gmail.com"),
        sign_in: true,
        count: 1,
    };

    let mut you = build_user(
        String::from("Ahmet"),
        String::from("ahmet123"),
        String::from("ahmet@miail.com"),
    );

    let mut others = User {
        name: String::from("Burak"),
        username: String::from("Burak123"),
        e_mail: String::from("burak@miail.com"),
        ..you
    };
    println!("{}", others.name);
}

fn build_user(name: String, username: String, e_mail: String) -> User {
    User {
        name,
        username,
        e_mail,
        sign_in: true,
        count: 1,
    }
}
