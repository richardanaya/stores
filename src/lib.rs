#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
pub use spin::Mutex;

pub struct Store<T, A>
where
    T: Default + Reduceable<A> + Sync + Send,
    A: Sync + Send,
{
    pub state: Arc<Mutex<T>>,
    watchers: Arc<Mutex<Vec<Box<dyn Fn(Arc<Mutex<T>>) -> () + Sync + Send>>>>,
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

    pub fn dispatch(&self, a: &A) -> Arc<Mutex<T>> {
        let s = T::reduce(self.state.clone(), a);
        for w in self.watchers.lock().iter() {
            w(s.clone())
        }
        s
    }

    pub fn watch<F>(&mut self, watcher: F)
    where
        F: 'static + Fn(Arc<Mutex<T>>) -> () + Sync + Send,
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
            state: Arc::new(Mutex::new(T::default())),
            watchers: Arc::new(Mutex::new(Vec::new())),
            phantom: core::marker::PhantomData,
        }
    }
}

pub trait Reduceable<A>
where
    A: Sync + Send,
{
    fn reduce(state: Arc<Mutex<Self>>, action: &A) -> Arc<Mutex<Self>>;
}
