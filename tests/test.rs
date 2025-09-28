use autoval::{Accessors, Builder, Init, Test};

#[derive(Accessors, Builder, Init, Test)]
struct User {
    name: String,
    email: Option<String>,
    #[default("20u8")]
    age: u8,
}

#[test]
pub fn testing() {
    let user = User::builder()
        .name(String::from("Matieuu"))
        .email(String::from("test@example.com"))
        .build();

    assert_eq!(user.is_ok(), true);
    let mut user = user.unwrap();
    assert_eq!(user.name.as_str(), "Matieuu");
    assert_eq!(user.email.is_some(), true);
    assert_eq!(user.email.as_ref().unwrap().as_str(), "test@example.com");
    assert_eq!(user.age, 20);

    user.set_name(String::from("Matusz"));
    user.set_email(String::from("example@example.com"));
    user.set_age(16);

    assert_eq!(user.name(), &String::from("Matusz"));
    assert_eq!(user.name_owned(), String::from("Matusz"));
    assert_eq!(user.email(), &Some(String::from("example@example.com")));
    assert_eq!(
        user.email_owned(),
        Some(String::from("example@example.com"))
    );
    assert_eq!(user.age(), &16);
    assert_eq!(user.age_owned(), 16);

    let user = User::new(String::from("Matieuu"));

    assert_eq!(user.name.as_str(), "Matieuu");
    assert_eq!(user.email.is_none(), true);
    assert_eq!(user.age, 20);
}
