use auto_val::Builder;

#[derive(Debug, Builder)]
#[validate]
struct User {
    #[size(min = 3, max = 20)]
    pub name: String,

    #[size(min = 18, max = 99)]
    pub age: u8,
}

#[test]
pub fn builder_validator_creates_user() {
    let user = User::builder()
        .name(String::from("Matieuu"))
        .age(20)
        .build();

    println!("Is ok");
    assert_eq!(user.is_ok(), true);
    let user = user.unwrap();

    println!("User name is: {}", &user.name);
    assert_eq!(user.name, String::from("Matieuu"));
    println!("User age is: {}", &user.age);
    assert_eq!(user.age, 20);

    // Check validator for too long name
    println!("Check validator for too long name");

    let user = User::builder()
        .name(String::from("MatieuuDestroyer213769"))
        .age(20)
        .build();

    println!("Is error");
    assert_eq!(user.is_err(), true);

    let err = user.err().unwrap();
    println!("Error content: {}", err);
    assert_eq!(err, "Field name is too long (max 20)");

    // Check validator for too big age
    println!("Check validator for too big age");

    let user = User::builder()
        .name(String::from("Matieuu"))
        .age(100)
        .build();

    println!("Is error");
    assert_eq!(user.is_err(), true);

    let err = user.err().unwrap();
    println!("Error content: {}", err);
    assert_eq!(err, "Field age is too big (max 99)");

    // Check validator for too short name
    println!("Check validator for too short name");

    let user = User::builder().name(String::from("Ma")).age(20).build();

    println!("Is error");
    assert_eq!(user.is_err(), true);

    let err = user.err().unwrap();
    println!("Error content: {}", err);
    assert_eq!(err, "Field name is too short (min 3)");

    // Check validator for too small age
    println!("Check validator for too small age");

    let user = User::builder()
        .name(String::from("Matieuu"))
        .age(10)
        .build();

    println!("Is error");
    assert_eq!(user.is_err(), true);

    let err = user.err().unwrap();
    println!("Error content: {}", err);
    assert_eq!(err, "Field age is too small (min 18)");
}
