# Stores

Reducable stores inspired by Redux

```rust
struct Counter {
    v:u32
}

enum Action{
    Increment
}

impl Reduceable<Action> for Counter {
    fn reduce(state:Arc<Mutex<Self>>,&action:Action) -> Arc<Mutex<Self>>{
        let mut s = state.lock();
        match action {
            _ => {
                s.v += 1;
            }
        }
        state.clone()
    }
}

fn main() {
    let r = Store::<Counter,Action>::get().lock();
    r.watch(|state|{
        println!("state changed! {:?}", state);
    });
    r.dispatch(&Action::Increment);
}
```