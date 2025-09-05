use autoval::Autoval;

#[derive(Autoval)]
#[autoval(builder)]
struct User {
    name: String,
    email: Option<String>,
    age: u8,
}

#[test]
pub fn testing() {
    let user = User::builder()
        .name(String::from("Matieuu"))
        .email(Some(String::from("test@example.com")))
        .age(20)
        .build();

    assert_eq!(user.is_ok(), true);
    let user = user.unwrap();
    assert_eq!(user.name.as_str(), "Matieuu");
    assert_eq!(user.email.is_some(), true);
    assert_eq!(user.email.unwrap().as_str(), "test@example.com");
    assert_eq!(user.age, 20);
}
