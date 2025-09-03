use auto_val::Builder;
#[derive(Debug, Builder)]
struct User {
    pub name: String,
    pub age: u8,
    pub male: bool,
}

#[test]
fn builder_creates_user() {
    let user = User::builder()
        .name(String::from("Matieuu"))
        .age(20)
        .male(true)
        .build()
        .unwrap();

    assert_eq!(user.name, String::from("Matieuu"));
    assert_eq!(user.age, 20);
    assert_eq!(user.male, true);
}
