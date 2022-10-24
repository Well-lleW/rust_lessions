fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    let ready:i32 = READY_AMOUNT;
    let missiles :i32 = STARTING_MISSILES;
    println!("Firing {} of my {} missiles", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
    READY_AMOUNT = 1;
}
