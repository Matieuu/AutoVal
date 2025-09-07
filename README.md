# AutoVal

**AutoVal** is lightweight rust crate that automatically generates **getters, setters, builders** and allows **fields validation** in a declarative way. Java libraries **Lombok** and **Jakarta Validator** has inspired me to create this project and make it in an idiomatic way for **Rust**.

---

## Functionalities

- Getters and Setters generation for struct
- Builder for ergonomic way of creating objects
- Declarative fields validation (size, regex, content, date)
- Validation service during object creation

---

## Przykład użycia

```rust
use std::{
    collections::HashMap,
    time::{Date, Duration, OffsetDateTime, PrimitiveDateTime}
};
use autoval::Autoval;

#[derive(Autoval)]
#[autoval(getters, setters, builder, init, validate)]
struct User {
    // len argument in size attribute means string length
    #[size(20 > len > 3)] // self explained - name len() between 3 and 20
    #[regex = r#"^[A-Z]\w+$"#] // regex that name should match
    // Instead one could use #[content(capitalize)] if wanted to make name capitalized always, no matter what letter cases are given
    #[content(notblank)] // content of the name cannot be blank
    name: String,

    // validation for Option<_> fields works only while Some(_)
    // assign #[some] if you want to assert field is always Some(_)
    #[email] // checks whether it is valid email or not
    #[content(notblank, lowercase)] // content of email cannot be blank and will be modified to be lowercase
    email: Option<String>,

    // val argument in size attribute means value of its number (signed / unsigned integers and floats)
    // its preffered to write #[size(18u8 < val < 99u8)] assigning specific type for comparator but without it still should work in most cases
    #[size(18 < val < 99)] // checks whether age meets up requirements
    age: u8,

    #[size(size > 1)] // checks whether Vec has more that one element
    hobbies: Vec<Hobby>,

    #[content(notempty)] // checks whether HashMap is not empty, meaning has at least one element
    friends: HashMap<String, Hobby>,

    // val argument in date attribute means date saved in argument
    #[date(val <= now)] // checks whether date is past or present compared to now
    birthday: OffsetDateTime,

    #[date(val >= now)] // checks if value is in future or now
    #[default = "Option::None"]
    deactivate_at: Option<Date>,

    subscription_start: PrimitiveDateTime,

    #[default = "Duration::week(1)"]
    // if during Builder value is not specified then Builder uses default value
    // Init now takes this parameter as Optional<_> and given None uses default value
    subscription_duration: Duration,
}

struct Hobby { .. }

fn main() -> Result<(), String> {
    let user = User::builder()
        .name(String::from("Matieuu"))
        .email("test@example.com")
        .age(20)
        .hobbies(vec![Hobby { "learning languages" }, Hobby { "playing games" }])
        .friends(map! { "Alone" => Hobby { .. } })
        .birthday(datetime!(2005-08-16 5:00 UTC))
        .subscription_start(datetime!(2025-09-05 13:00))
        .build()?; // here validation starts working

    let user = User::new(UserInit {
        name: String::from("Matieuu"),
        email: "test@example.com",
        age: 20,
        hobbies: vec![Hobby { "learning languages" }, Hobby { "playing games" }],
        friends: map! { "Alone" => Hobby { .. } },
        birthday: datetime!(2005-08-16 5:00 UTC),
        subscription_start: datetime!(2025-09-05 13:00),
        ..Default::default()
    })?;

    Ok(())
}
```

---

## Next steps

- [x] Create main autoval attribute
- [ ] Implement creation of:
    - [x] Builder
    - [x] Init
    - [x] Setter
    - [x] Getter
    - [ ] Validator
- [ ] Implement field attributes:
    - [ ] size
    - [ ] content
    - [ ] date
    - [ ] default
    - [ ] email
    - [ ] regex

---

## Instalation

Add to `Cargo.toml`:

```toml
[dependencies]
autoval = "0.1.0"
```
