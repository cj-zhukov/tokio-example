use tokio::time::{sleep, Duration};
use rand::{thread_rng, Rng};
use tokio::task::JoinSet;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let max_concurrent = 2;
    let ids = ["1", "x", "3", "4", "5", "6", "7", "8", "9", "a"];
    let mut tasks = JoinSet::new();
    
    for id in ids {
        while tasks.len() >= max_concurrent {
            if let Some (res) = tasks.join_next().await {
                match res {
                    Ok(_) => (),
                    Err(e) => println!("failed running foo: {}", e)
                }
            }
        }
        tasks.spawn(foo(id));
    }
    
    println!("done spawning");
    
    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(_) => (),
            Err(e) => println!("failed running foo: {}", e)
        }
    }
    
    println!("all done");

    Ok(())
}

async fn foo(x: &str) {
    println!("start processing input: {}", x);
    let num: u64 = thread_rng().gen_range(0..=10);
    sleep(Duration::from_secs(num)).await;
}