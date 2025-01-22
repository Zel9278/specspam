use std::{thread,env};
use rand::{Rng, SeedableRng};

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    let mut disable_cpu = false;
    let mut disable_memory = false;

    for arg in args {
        if arg == "--disable-cpu" {
            disable_cpu = true;
        } else if arg == "--disable-memory" {
            disable_memory = true;
        }
    }

    if disable_cpu && disable_memory {
        println!("You can't disable both cpu and memory at the same time.");
        return;
    }

    if !disable_memory {
        let _handle = thread::spawn(|| {
            println!("Thread 1 - Memory: [MEMORY BURNER] Hello from a new thread!");
            let mut buffer = Vec::new();

            let desired_bytes = 1024000000000;

            println!("[MEMORY BURNER] Allocating buffer with {} bytes", desired_bytes);

            while buffer.len() < desired_bytes {
                buffer.push(0);
            }
        
            println!("[MEMORY BURNER] Buffer size: {} bytes", buffer.len());
        });   
    }


    if !disable_cpu {
        let _handle2 = thread::spawn(|| {
            println!("Thread 2 - CPU: [CPU BURNER] Hello from another new thread!");
            let cpus = num_cpus::get();
            println!("[CPU BURNER] Your cpu count: {}", cpus);
            println!("[CPU BURNER] Now burning you cpu! eat cpu now!");
            for _ in 0..cpus - 1 {
                std::thread::spawn(omoishori);
            }

            fn omoishori() {
                let mut rng = rand::rngs::StdRng::from_seed(*b"kazukazu123123____kazukazu123123");
                loop {
                    for _ in 0..100000000 {
                        let _: u128 = rng.gen();
                    }
                }
            }
        });
    }

    std::thread::sleep(std::time::Duration::MAX);
    println!("[MAIN] Back in the main thread.");
}
