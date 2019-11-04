use std::sync::Arc;
pub use spin::Mutex;

#[derive(Debug)]
pub struct Store<T,A> where T:Default+Reduceable<A>+Sync+Send,A:Sync+Send{
    pub state:Arc<Mutex<T>>,
    phantom: std::marker::PhantomData<A>
}

impl<T,A> Store<T,A> where T:Default+Reduceable<A>+Sync+Send,A:Sync+Send{
    pub fn get() -> &'static Mutex<Self>{
        globals::get()
    }
    
    pub fn reduce(&self,a:&A) -> Arc<Mutex<T>>{
        T::reduce(self.state.clone(),a)
    }
}

impl<T,A> Default for  Store<T,A> where T:Default+Reduceable<A>+Sync+Send,A:Sync+Send{
    fn default() -> Self {
        Store::<T,A>{
            state:Arc::new(Mutex::new(T::default())),
            phantom: std::marker::PhantomData
        }
    }
}

pub trait Reduceable<A> where A:Sync+Send {
    fn reduce(state:Arc<Mutex<Self>>,action:&A) -> Arc<Mutex<Self>>;
}
