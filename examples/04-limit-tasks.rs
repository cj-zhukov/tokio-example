use tokio::time::{sleep, Duration};
use rand::{thread_rng, Rng};
use tokio::task::JoinSet;   

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const MAX_CONCURRENT_TASKS: usize = 2; 

 
#[tokio::main]
async fn main() -> Result<()> {
    let ids = ["1", "x", "3", "4", "5", "6", "7", "8", "9", "a"];
    let mut tasks = JoinSet::new();
    
    for id in ids {
        tasks.spawn(foo(id));

        if tasks.len() == MAX_CONCURRENT_TASKS {
            if let Some (res) = tasks.join_next().await {
                match res {
                    Ok(res) => match res {
                        Ok(_) => (),
                        Err(e) => println!("failed running foo: {}", e)
                    },
                    Err(e) => println!("failed running task: {}", e)
                }
            }
        }
    }

    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(res) => match res {
                Ok(_) => (),
                Err(e) => println!("failed running foo: {}", e)
            },
            Err(e) => println!("failed running task: {}", e)
        }
    }

    Ok(())
}

async fn foo(x: &str) -> Result<()> {
    println!("start processing input: {}", x);
    let num: u64 = thread_rng().gen_range(0..=10);
    sleep(Duration::from_secs(num)).await;
    let res = x.parse::<i32>()?;
    println!("{}", res);
    Ok(())
}