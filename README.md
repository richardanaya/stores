# Stores

Reducable stores inspired by Redux

```rust
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