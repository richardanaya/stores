use std::sync::Arc;
use stores::*;

#[derive(Default, Debug)]
struct Counter {
    v: u32,
}

#[derive(Debug)]
enum Action {
    Increment,
}

impl Reduceable<Action> for Counter {
    fn reduce(state: State<Self>, action: &Action) -> State<Self> {
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
    println!("{:?}", r.state.lock());
    r.dispatch(&Action::Increment);
    println!("{:?}", r.state.lock());
    r.dispatch(&Action::Increment);
    println!("{:?}", r.state.lock());
}
