#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
pub use spin::Mutex;

pub struct State<S> {
    state:Arc<Mutex<S>>
}

impl<S> State<S> {
    pub fn new(s:S) -> Self{
        State {
            state:Arc::new(Mutex::new(s))
        }
    }
    pub fn lock(&self) -> spin::MutexGuard<S> {
        self.state.lock()
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            state:self.state.clone()
        }
    }
}

pub struct Store<T, A>
where
    T: Default + Reduceable<A> + Sync + Send,
    A: Sync + Send,
{
    pub state: State<T>,
    watchers: Arc<Mutex<Vec<Box<dyn Fn(State<T>) -> () + Sync + Send>>>>,
    phantom: core::marker::PhantomData<A>,
}

impl<T, A> Store<T, A>
where
    T: Default + Reduceable<A> + Sync + Send,
    A: Sync + Send,
{
    pub fn get() -> &'static Mutex<Self> {
        globals::get()
    }

    pub fn dispatch(&mut self, a: &A) -> State<T> {
        let s = T::reduce(self.state.clone(), a);
        self.state = s.clone();
        for w in self.watchers.lock().iter() {
            w(self.state.clone())
        }
        s
    }

    pub fn watch<F>(&mut self, watcher: F)
    where
        F: 'static + Fn(State<T>) -> () + Sync + Send,
    {
        self.watchers.lock().push(Box::new(watcher))
    }
}

impl<T, A> Default for Store<T, A>
where
    T: Default + Reduceable<A> + Sync + Send,
    A: Sync + Send,
{
    fn default() -> Self {
        Store::<T, A> {
            state: State::new(T::default()),
            watchers: Arc::new(Mutex::new(Vec::new())),
            phantom: core::marker::PhantomData,
        }
    }
}

pub trait Reduceable<A>
where
    A: Sync + Send,Self:Sized
{
    fn reduce(state: State<Self>, action: &A) -> State<Self>;
}
