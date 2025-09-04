use auto_val::Builder;

#[derive(Debug, Builder)]
#[validate]
struct User {
    #[size(max = 20)]
    pub name: String,

    #[size(min = 18, max = 99)]
    pub age: u8,
}

#[test]
pub fn builder_validates_creates_user() {
    let user = User::builder()
        .name(String::from("Matieuu"))
        .age(20)
        .build();

    assert_eq!(user.is_ok(), true);
    let user = user.unwrap();
    assert_eq!(user.name, String::from("Matieuu"));
    assert_eq!(user.age, 20);

    let user = User::builder()
        .name(String::from("MatieuuDestroyer213769"))
        .age(20)
        .build();

    assert_eq!(user.is_err(), true);
    assert_eq!(
        user.err().unwrap().as_str(),
        "Field name is too long (max 20)"
    );

    let user = User::builder()
        .name(String::from("Matieuu"))
        .age(100)
        .build();

    assert_eq!(user.is_err(), true);
    assert_eq!(
        user.err().unwrap().as_str(),
        "Field age is too big (max 99)"
    );

    // let user = User::builder()
    //     .name(String::from("Matieuu"))
    //     .age(10)
    //     .build();

    // assert_eq!(user.is_err(), true);
    // assert_eq!(
    //     user.err().unwrap().as_str(),
    //     "Field age is too small (min 10)"
    // );
}
