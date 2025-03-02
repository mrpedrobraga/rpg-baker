use futures_signals::signal::Mutable;
use rpg_baker::scripting::{BlockExt as _, standard::*};

fn main() {
    let ia = Mutable::new(Box::new(Int(1)) as Box<dyn Block<Output = i32>>);
    let ib = Mutable::new(Box::new(Int(2)) as Box<dyn Block<Output = i32>>);
    let mut sum = And::create();

    sum.slot_a
        .try_place(Box::new(ia))
        .expect("Failure placing first operand.");
    sum.slot_b
        .try_place(Box::new(ib))
        .expect("Failure placing second operand.");

    let result = sum.evaluate();
    dbg!(result);
}
