use std::sync::Arc;
use stores::*;

#[derive(Default,Debug)]
struct Counter {
    v: u32,
}

enum Action {
    Increment,
}

impl Reduceable<Action> for Counter {
    fn reduce(state: Arc<Mutex<Self>>, action: &Action) -> Arc<Mutex<Self>> {
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
    let mut r = Store::<Counter, Action>::get().lock();
    r.watch(|state|{
        println!("{:?}", state);
    });
    r.reduce(&Action::Increment);
    r.reduce(&Action::Increment);
}
