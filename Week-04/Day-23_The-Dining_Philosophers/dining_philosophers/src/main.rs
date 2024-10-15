use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct DiningPhilosophers {
    forks: Vec<Arc<(Mutex<bool>, Condvar)>>,
}

impl DiningPhilosophers {
    fn new(count: usize) -> Self {
        let forks = (0..count).map(|_| Arc::new((Mutex::new(true), Condvar::new()))).collect();
        DiningPhilosophers { forks }
    }

    fn wants_to_eat(
        &self,
        philisopher: usize,
        pick_left_fork: impl Fn(),
        pick_right_fork: impl Fn(),
        eat: impl FnOnce(),
        put_left_fork: impl Fn(),
        put_right_fork: impl Fn(),
    ) {
        let left_fork = philisopher;
        let right_fork = (philisopher + 1) % self.forks.len();

        // Ensure philosophers pick forks in a consistent order to prevent deadlocks
        let (first_fork, second_fork) = if philisopher % 2 == 0 {
            (left_fork, right_fork)
        } else {
            (right_fork, left_fork)
        };

        // Pick up first fork
        let (lock, cvar) = &*self.forks[first_fork];
        let mut fork = lock.lock().unwrap(); // avoid deadlock
        while !*fork {
            fork = cvar.wait(fork).unwrap();
        }
        *fork = false;
        drop(fork);
        if first_fork == left_fork {
            pick_left_fork();
        } else {
            pick_right_fork();
        }

        // Pick up second fork
        let (lock, cvar) = &*self.forks[second_fork];
        let mut fork = lock.lock().unwrap();
        while !*fork {
            fork = cvar.wait(fork).unwrap();
        }
        *fork = false;
        drop(fork);
        if second_fork == left_fork {
            pick_left_fork();
        } else {
            pick_right_fork();
        }

        // Eat
        eat();

        // Put down forks
        *self.forks[first_fork].0.lock().unwrap() = true;
        self.forks[first_fork].1.notify_one();
        if first_fork == left_fork {
            put_left_fork();
        } else {
            put_right_fork();
        }

        *self.forks[second_fork].0.lock().unwrap() = true;
        self.forks[second_fork].1.notify_one();
        if second_fork == left_fork {
            put_left_fork();
        } else {
            put_right_fork();
        }
    }
}

fn main() {
    let count = 5;
    let philosophers = Arc::new(DiningPhilosophers::new(count));
    let mut handles = vec![];

    for i in 0..count {
        let philosophers = Arc::clone(&philosophers);
        handles.push(
            thread::spawn(move || {
                philosophers.wants_to_eat(
                    i,
                    || println!("Philosopher {} picks left fork", i),
                    || println!("Philosopher {} picks right fork", i),
                    || println!("Philosopher {} eats", i),
                    || println!("Philosopher {} puts down left fork", i),
                    || println!("Philosopher {} puts down right fork", i),
                )
            })
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }
}