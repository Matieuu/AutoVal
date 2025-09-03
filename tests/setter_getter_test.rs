use auto_val::{Getter, Setter};

#[derive(Debug, Default, Setter, Getter)]
struct User {
    name: String,
    age: u8,
    male: Option<bool>,
}

#[test]
fn builder_creates_user() {
    let mut user = User::default();

    user.set_name(String::from("Matieuu"));
    user.set_age(20);
    user.set_male(Some(true));

    assert_eq!(user.name(), &String::from("Matieuu"));
    assert_eq!(user.age(), &20);
    assert_eq!(user.male(), &Some(true));
}
