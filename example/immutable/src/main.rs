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
