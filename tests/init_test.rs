use auto_val::Init;

#[derive(Debug, Init)]
struct User {
    pub name: String,
    pub age: u8,
    pub male: Option<bool>,
}

#[test]
fn init_creates_user() {
    let user = User::new(String::from("Matieuu"), 20);

    assert_eq!(user.name, String::from("Matieuu"));
    assert_eq!(user.age, 20);
    assert_eq!(user.male, None);
}
