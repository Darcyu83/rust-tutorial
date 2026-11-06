fn main() {
    let n: u32 = 10;

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }

    println!("{n}번째 피보나치 수: {a}");
}
