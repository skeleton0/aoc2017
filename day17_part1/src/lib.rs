mod spinlock;

pub fn run() {
    let mut spinlock = spinlock::Spinlock::new();

    let solution = spinlock.solve_spinlock(324, 2017);

    println!("Solution returned from solve_spinlock(): {}", solution);
}