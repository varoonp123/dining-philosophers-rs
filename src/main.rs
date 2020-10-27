use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time;
use std::sync::Arc;

fn run_philosopher(philosopher_indx: usize, adj_forks: [Arc<AtomicBool>; 2]) {
    //Strategy: Odd indexed philosophers will pick up their left fork first when it becomes
    //available, then pick up the right fork when it becomes available. Then eat for a while then
    //put the left fork then the right fork back on the table. Even indexed philosophers will do
    //the same but starting with their right fork. This prevents deadlocks. The assertions are
    //purely sanity checks. Does this actually need sequential consistency?
    let starting_indx = philosopher_indx % 2;
    let first_fork = &adj_forks[starting_indx];
    let second_fork =& adj_forks[(starting_indx + 1) % 2];
    loop {
        while !first_fork.compare_and_swap(true, false, Ordering::SeqCst) {}
        assert_eq!(first_fork.load(Ordering::SeqCst), false);
        while !second_fork.compare_and_swap(true, false, Ordering::SeqCst) {}
        assert_eq!(first_fork.load(Ordering::SeqCst), false);
        assert_eq!(second_fork.load(Ordering::SeqCst), false);
        println!("Philosopher {} is eating!", philosopher_indx);
        thread::sleep(time::Duration::from_secs(1));
        println!("Philosopher {} Stopped eating!!", philosopher_indx);
        first_fork.compare_and_swap(false, true, Ordering::SeqCst);
        second_fork.compare_and_swap(false, true, Ordering::SeqCst);
        thread::sleep(time::Duration::from_secs(1));
    };
}
fn main() {
    //true means the fork is on the table/is available to be taken by a diner
    // These must be Arcs because currently the borrow checker cannot rule out the possibility that
    // pure AtomicBools will only live through the main function. They are different threads so
    // any references may live longer than this function.
    let forks: Vec<Arc<AtomicBool>> = vec![
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
        Arc::new(AtomicBool::new(true)),
    ];
    let mut threads = vec![];
    for indx in 0..5 {
        thread::sleep(time::Duration::from_millis(200));
        // Ard::clone because Arc types do not implement the clone trait. Calling this function
        // increases the ref count of each atomic bool.
        let fks: [Arc<AtomicBool>; 2] = [Arc::clone(&forks[indx % 5]), Arc::clone(&forks[(indx + 1) % 5])];
        threads.push(thread::spawn(move || run_philosopher(indx, fks)));
    }
    // Without joining a thread, the threads will be created then the program will end before
    // anything interesting happens. 
    threads.pop().unwrap().join();
}
