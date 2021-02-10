use threadpool::ThreadPool;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;
use std::process::Command;

use std::time::{Duration, Instant};


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn scrape_steam(id: u64) -> String
{

    //basically this just downloads a webpage to /tmp/MTWS/wp

    let mut path : String = "/tmp/MTWS/wp/".to_owned();
    let mut url: String = "https://steamcommunity.com/profiles/".to_owned();
    let id_s: String = id.to_string().to_owned();

    url.push_str(&id_s);
    
    let mut mkdir = Command::new("mkdir");
    mkdir.arg("-p");
    mkdir.arg("/tmp/MTWS/wp");
    
    let _out = mkdir.output();

    let mut wget = Command::new("wget");
    wget.arg(url);
    wget.current_dir("/tmp/MTWS/wp");
    let _out = wget.output();

    
    //parse the webpage
    //
    path.push_str(&id_s);
    if let Ok(lines) = read_lines(path) 
    {
        for line in lines 
        {   
            let line_w = line.unwrap();
            if line_w.contains("hours past 2 weeks")
            {   
                let mut new_url: String = "https://steamcommunity.com/profiles/".to_owned();
                let mut new_id_s: String = id.to_string().to_owned();
                new_url.push_str(&new_id_s);

                println!("{:?}",new_url);
                println!("{:?}",&line_w[12..line_w.len()-5]);
                return line_w;
            }
        }
    }

    return "None".to_string();
}


fn find_steam_users()
{
    // create at least as many workers as jobs or you will deadlock yourself
    let n_workers = 30;
    let n_jobs = 24;
    let pool = ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0));
    let profile_count = Arc::new(AtomicUsize::new(0));

    assert!(n_jobs <= n_workers, "too many jobs, will deadlock");

    // create a barrier that waits for all jobs plus the starter thread
    let barrier = Arc::new(Barrier::new(n_jobs + 1));


    for thread_id in 0..n_jobs {
        
        let an_atomic = Arc::clone(&an_atomic);
        let profile_count = Arc::clone(&profile_count);
        let barrier = barrier.clone();

        pool.execute(move|| {
            
            let mut num = rand::thread_rng().gen_range(76561198000000000..76561198999999999);
            let mut found: bool = false;


            while !found && an_atomic.load(Ordering::SeqCst) == 0
            {
                profile_count.fetch_add(1, Ordering::SeqCst);
                //println!("{:?} :: {:?}",thread_id, num);
                if scrape_steam(num) != "None"
                {
                    found = true;
                    println!("{:?} FOUND! {:?} searched {:?} profiles",thread_id ,num, profile_count);
                }
                num = rand::thread_rng().gen_range(76561198000000000..76561198999999999);
            }
            
            an_atomic.fetch_add(1, Ordering::SeqCst);
           // println!("job: {:?} finished", thread_id);
            

            barrier.wait();
        });
    }

    barrier.wait();
}


fn main()
{   
    let now = Instant::now();

    find_steam_users();

    println!("took {:?}s", now.elapsed().as_secs());

}




