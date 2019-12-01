mod spinlock;

pub fn run() {
    let mut spinlock = spinlock::Spinlock::new();

    let solution = spinlock.solve_spinlock_fast(324, 50000000);

    println!("Solution returned from solve_spinlock(): {}", solution);
}