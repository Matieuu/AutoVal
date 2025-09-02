# AutoVal

**AutoVal** to lekka biblioteka w Rust, która automatycznie generuje **gettery, settery, buildery** oraz umożliwia **walidację pól** w sposób deklaratywny. Inspiracją były biblioteki **Lombok** oraz **Jakarta Validator** z Javy, jednak w pełni idiomatyczna w Rust.

---

## Funkcjonalności

- Generowanie getterów i setterów dla struct
- Builder dla ergonomicznego tworzenia obiektów
- Deklaratywna walidacja pól (min, max, regex, required)
- Obsługa walidacji podczas budowy obiektu
- Możliwość rozbudowy o async walidatory

---

## Przykład użycia

```rust
use autoval::prelude::*;

#[derive(Getters, Setters, Builder, Validate)]
struct User {
    #[validate(min = 18)]
    age: u8,

    #[validate(regex = r"^\w+@\w+\.\w+$")]
    email: String,
}

fn main() -> Result<(), String> {
    let user = User::builder()
        .age(25)
        .email("alice@example.com")
        .build()?; // walidacja automatyczna

    println!("User email: {}", user.get_email());
    Ok(())
}
````

---

## Instalacja

Dodaj do `Cargo.toml`:

```toml
[dependencies]
autoval = { path = "../autoval" }
```
