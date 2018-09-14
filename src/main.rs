// GOAL: 
//  Setup n threads, where n is the number of CPUs on the host.
//  Each thread will run a Local futures executor pool.
//  One thread will coordinate a test:
//      1. Loop (n-1) times, each pass
//      2. Define a future which when run captures and returns its cpu id
//      3. Print the value of each future, should be the same as main thread
//      4. Re-run loop, but this time send the future to specific CPU pool to run
//      5. Now the returned id should be that CPU
extern crate crossbeam_channel;
extern crate futures;
extern crate hwloc;

use hwloc::{Topology, ObjectType};

fn num_cpus() -> usize {
    Topology::new()
        .objects_with_type(&ObjectType::Core)
        .unwrap()
        .len()
}

fn main() {
    // Setup our threads
    println!("Num Cores: {}", num_cpus());
}
