pub mod task;

pub struct RoundRobinWorkerPool {}
impl RoundRobinWorkerPool {
    pub fn new() -> RoundRobinWorkerPool {
        RoundRobinWorkerPool {}
    }

    pub async fn get(&self) -> WorkerId {
        WorkerId {}
    }
}

pub struct WorkerId {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
