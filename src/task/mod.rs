use std::future::Future;

pub fn block_on<F, T>(_future: F) -> T
where
    F: Future<Output = T>,
{
    unimplemented!();
}

pub fn spawn_worker<F, T>(_worker: super::WorkerId, _future: F) -> JoinHandle
where
    F: Future<Output = T> + Send,
{
    unimplemented!();
}

pub fn spawn_local<F, T>(_future: F) -> JoinHandle
where
    F: Future<Output = T>,
{
    unimplemented!();
}

pub struct JoinHandle {}
