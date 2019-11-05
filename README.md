# Stores

Reducable stores inspired by Redux

```toml
[dependencies]
stores = "0.1.0"
```

```rust
#[derive(Default)]
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
        let prev = &*state.lock();
        match action {
            Increment => {
                return State::new(Counter{
                    v:prev.v+1,
                    ..*prev
                })
            },
            Decrement => {
                return State::new(Counter{
                    v:prev.v-1,
                    ..*prev
                })
            }
        }
        state.clone()
    }
}

fn main() {
    let r = Store::<Counter,Action>::get().lock();
    r.watch(|state|{
        println!("state changed! {:?}", state.lock());
    });
    r.dispatch(&Action::Increment);
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
