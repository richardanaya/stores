# Stores

Reducable stores inspired by Redux

```toml
[dependencies]
stores = "0.1.0"
```

```rust
use stores::*;

#[derive(Default,Debug)]
struct Counter {
    v: u32,
}

enum Action {
    Increment,
    Decrement,
    Nothing,
}

impl Reduceable<Action> for Counter {
    fn reduce(state: State<Self>, action: &Action) -> State<Self> {
        match action {
            Action::Increment => {
                let prev = state.lock();
                State::new(Counter{
                    v:prev.v+1,
                    ..*&*prev
                })
            },
            Action::Decrement => {
                let prev = state.lock();
                State::new(Counter{
                    v:prev.v-1,
                    ..*&*prev
                })
            },
            _ => state
        }
    }
}

fn main() {
    let mut r = Store::<Counter, Action>::get().lock();
    r.watch(|state|{
        println!("{:?}", state.lock());
    });
    r.dispatch(&Action::Increment);
    r.dispatch(&Action::Decrement);
    r.dispatch(&Action::Nothing);
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in stores by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
