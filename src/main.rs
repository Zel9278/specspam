use std::thread;
use rand::{Rng, SeedableRng};

fn main() {
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
    
        std::thread::sleep(std::time::Duration::MAX);
        println!("[MAIN] Back in the main thread.");
}