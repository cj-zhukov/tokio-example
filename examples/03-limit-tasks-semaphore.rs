use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};
use rand::{thread_rng, Rng};

type SemaphoreRef = Arc<Semaphore>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const MAX_CONCURRENT_TASKS: usize = 2; 

#[tokio::main]
async fn main() -> Result<()> {
    let ids = ["1", "x", "3", "4", "5", "6", "7", "8", "9", "a"];
    let semaphore = Semaphore::new(MAX_CONCURRENT_TASKS); 
    let semaphore = Arc::new(semaphore);
    let mut tasks = vec![];
    let mut outputs = vec![];
    for id in ids {
        let task = tokio::spawn(foo(semaphore.clone(), &id));
        tasks.push(task);
    }

    for task in tasks {
        match task.await {
            Ok(res) => match res {
                Ok(val) => outputs.push(val),
                Err(e) => println!("failed processing data: {}", e)
            },
            Err(e) => println!("failed running foo: {}", e)
        }
    }

    println!("all done");
    println!("{:?}", outputs);

    Ok(())
}

async fn foo(semaphore: SemaphoreRef, x: &str) -> Result<i32> {
    let _permit = semaphore.acquire().await?; 
    println!("start processing input: {}", x);
    let num: u64 = thread_rng().gen_range(0..=10);
    sleep(Duration::from_secs(num)).await;

    Ok(x.parse::<i32>()?)
}