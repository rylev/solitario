use async_std::io;
use async_std::net;
use async_std::prelude::*;
use solitario::{task, RoundRobinWorkerPool};

fn main() -> Result<(), Exception> {
    task::block_on(async {
        let listener = net::TcpListener::bind(("127.0.0.1", 8080)).await?;
        let addr = format!("http://{}", listener.local_addr()?);
        println!("listening on {}", addr);
        let mut incoming = listener.incoming();
        // create a worker pool
        let pool = RoundRobinWorkerPool::new();

        while let Some(stream) = incoming.next().await {
            // get the next available worker
            let worker = pool.get().await;
            // spawn that worker
            task::spawn_worker(worker, async {
                // This async block is `Send` and therefore any work
                // that requires access to items sent from the main thread
                // can be done here, but this is still guranteed to happen
                // on the worker thread
                task::spawn_local(async {
                    // This async block is !Send and thus does not require
                    // locks around shared state (since state cannot be shared here
                    let stream = stream.unwrap();
                    let (reader, writer) = &mut (&stream, &stream);
                    io::copy(reader, writer).await.unwrap();
                })
            });
        }
        Ok(())
    })
}

type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;
