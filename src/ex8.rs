// Example 8: closures, (bounded) polymorphism

fn twice_i32(x0: i32, f: |arg: i32| -> i32) -> i32 {
    // closure named `f`                 ^
    //                                   |
    //     ... with type ~~~~~~~~~~~~~~~~^

    let x1 = f(x0); // (work-around weakness in current borrow-checker)

    f(x1)
}

pub fn main() {
    println!("twice_i32(0, add_1): {}", twice_i32(0, |y| y+1));
    let w = 3;
    println!("twice_i32(0, add_w): {}", twice_i32(0, |z| z+w));

    println!("twice(0, add_1): {}", twice(0i32, |y| y+1));

    // println!("twice_peano(0): {}", twice_peano(0i32)); // XXX (see exercise below)
}

pub fn twice<X>(x: X, f: |arg: X| -> X) -> X {
    let x1 = f(x);
    f(x1)
}

trait Peano {
    fn succ(self) -> Self;
}

#[cfg(exercise_for_reader)]
impl Peano for i32 {

}

pub fn twice_peano<X:Peano>(x: X) -> X {
    unimplemented!()

    // Hint: review ex4.rs
}

// EXERCISE: finish implementing Peano for `i32`; uncomment line marked XXX above.

// EXERCISE: The first two calls to `twice_i32` in `main` illustrate
// closures closed over their environment.  Try to make the classic
// `make_adder` example of a higher-order function, i.e. defined
// analogously to `make_adder ≡ λw.λz.z+w`
//
// Discuss.
