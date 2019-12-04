use std::future::Future;

pub fn block_on<F, T>(_future: F) -> T
where
    F: Future<Output = T>,
{
    unimplemented!();
}

pub fn spawn_on_worker<F, T>(_future: F)
where
    F: Future<Output = T>,
{
    unimplemented!();
}
