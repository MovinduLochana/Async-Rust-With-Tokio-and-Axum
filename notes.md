### Commands
- `cargo watch -q -c -w src/ -x run`
- `cargo watch -q -c -w tests/ -x "test -q simple_tests -- --nocapture"`

```http request
GET http://localhost:8080/params?name=movindu
```

FromRef

is a trait used to extract specific "substates" 
from your main application state, allowing your route
handlers to only access the exact data they need rather
than the entire global state.Why use FromRefInstead of 
having all your handlers depend on a massive AppState struct,
FromRef teaches Axum how to derive a smaller, 
specific type from a reference to the main state.
This improves code modularity and makes unit testing
individual handlers significantly easier. 

You can implement the FromRef trait manually or automatically 
using the #[derive(FromRef)] macro.

```rust
use axum::{extract::{State, FromRef}, Router, routing::get};

// Automatically derive FromRef to extract subfields
#[derive(Clone, FromRef)]
struct AppState {
    database_pool: DatabasePool,
    api_key: String,
}

// Handlers now receive only the necessary part of the state
async fn db_handler(State(db): State<DatabasePool>) { /* ... */ }
async fn api_handler(State(key): State<String>) { /* ... */ }

```

Custom

```rust
impl FromRef<AppState> for DatabasePool {
    fn from_ref(app_state: &AppState) -> DatabasePool {
        app_state.database_pool.clone()
    }
}
```

### `regex` vs. `lazy_regex` Crates
Instantiate and store compiled regular expressions.

The Standard `regex` Crate 
The foundational crate that provides the underlying 
pattern-matching engine. Directly using `Regex::new(r"pattern")`
requires compiling the expression every time it runs 
(which is slow). To avoid this overhead, 
you have to manually handle global state variables 
using `once_cell` or `std::sync::LazyLock`.

- The `lazy_regex` Crate
A convenient wrapper crate built on top of the standard
`regex` crate. It offers procedural macros 
(like `regex!(r"pattern")` or `lazy_regex!(r"pattern")`) 
that automatically compile your pattern at compile-time
and lazily lock it into a static reference upon 
the first function call.

- Why `lazy_regex` is preferred
  - Performance: Automatically compiles expressions only
  once across your entire application execution. 
  
  - Compile-time Check: Errors in your regex syntax are
  caught during `cargo check` instead of
  panicking at runtime.

  - Syntax Cleanliness: Eliminates the boilerplate required 
  to set up manual lazy evaluations or builders

prev Ctx

```rust
// Extract cookies
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies
            .get(AUTH_TOKEN)
            .map(|token| token.value().to_string());

        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthToken)
            .and_then(parse_token)?;

        // TODO: Token Validation
```
