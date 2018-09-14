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
extern crate libc;

use hwloc::{Topology, ObjectType, CPUBIND_THREAD};
use std::thread;

fn num_cpus() -> usize {
    Topology::new()
        .objects_with_type(&ObjectType::Core)
        .unwrap()
        .len()
}

fn main() {
    let ncpus = num_cpus();
    let chans : Vec<_> = (0 .. ncpus)
        .map(|_| crossbeam_channel::unbounded())
        .collect();

    // Setup our threads -- CPU0 will be our main thread
    let _handles : Vec<_> = (1 .. ncpus)
        .map(|cpu| {
            let r = chans[cpu].1.clone();
            let s : Vec<_> = chans.iter().map(|(s,_)| s.clone()).collect();
            thread::spawn(move || {
                let tid = unsafe { libc::pthread_self() };
                let mut topo = Topology::new();
                let cpuset = {
                    let cores = topo.objects_with_type(&ObjectType::Core).unwrap();
                    let mut cpuset = match cores.get(cpu) {
                        Some(val) => val.cpuset().unwrap(),
                        None => panic!("No core found with id {}", cpu)
                    };
                    cpuset.singlify();
                    cpuset
                };
                topo.set_cpubind_for_thread(tid, cpuset, CPUBIND_THREAD).unwrap();

                match r.recv() {
                    Some(cpu_from) => println!("Thread {} on cpu {} from {}.", tid, cpu, cpu_from),
                    None => println!("Recieve side closed")
                }

                s[0].send(cpu);
            });
        })
        .collect();

    for cpu in 1 .. ncpus {
        chans[cpu].0.send(0);
        match chans[0].1.recv() {
            Some(cpu_from) => println!("Main thread from {}", cpu_from),
            None => println!("Main recieve side closed")
        }
    }

    drop(chans);
}
