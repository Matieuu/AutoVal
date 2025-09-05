use autoval::Autoval;

#[derive(Autoval)]
#[autoval(builder, setters)]
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
    let mut user = user.unwrap();
    assert_eq!(user.name.as_str(), "Matieuu");
    assert_eq!(user.email.is_some(), true);
    assert_eq!(user.email.as_ref().unwrap().as_str(), "test@example.com");
    assert_eq!(user.age, 20);

    user.set_name(String::from("Matusz"));
    user.set_email(Some(String::from("example@example.com")));
    user.set_age(16);

    assert_eq!(user.name().as_str(), "Matusz");
    assert_eq!(user.email().is_some(), true);
    assert_eq!(
        user.email().as_ref().unwrap().as_str(),
        "example@example.com"
    );
    assert_eq!(user.age(), &16);
}
