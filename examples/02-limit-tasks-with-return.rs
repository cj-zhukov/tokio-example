use tokio::time::{sleep, Duration};
use rand::{thread_rng, Rng};
use tokio::task::JoinSet;   

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const MAX_CONCURRENT_TASKS: usize = 3; 

 
#[tokio::main]
async fn main() -> Result<()> {
    let ids = ["1", "x", "3", "4", "5", "6", "7", "8", "9", "a"];
    let mut tasks = JoinSet::new();
    let mut outputs = vec![];
    
    for id in ids {
        while tasks.len() >= MAX_CONCURRENT_TASKS {
            if let Some (res) = tasks.join_next().await {
                match res {
                    Ok(res) => match res {
                        Ok(val) => outputs.push(val),
                        Err(e) => println!("failed processing data: {}", e)
                    },
                    Err(e) => println!("failed running task: {}", e)
                }
            }
        }
        tasks.spawn(foo(id));
    }
    
    println!("done spawning");
    
    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(res) => match res {
                Ok(val) => outputs.push(val),
                Err(e) => println!("failed processing data: {}", e)
            },
            Err(e) => println!("failed running task: {}", e)
        }
    }
    
    println!("all done");
    println!("{:?}", outputs);

    Ok(())
}

async fn foo(x: &str) -> Result<i32> {
    println!("start processing input: {}", x);
    let num: u64 = thread_rng().gen_range(0..=10);
    sleep(Duration::from_secs(num)).await;

    Ok(x.parse::<i32>()?)
}