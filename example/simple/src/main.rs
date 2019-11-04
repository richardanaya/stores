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
    let r = Store::<Counter, Action>::get().lock();
    println!("{:?}", r.state);
    r.reduce(&Action::Increment);
    println!("{:?}", r.state);
    r.reduce(&Action::Increment);
    println!("{:?}", r.state);
}
