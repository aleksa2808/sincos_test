fn main() {
    let a: f32 = 0.7568419;

    let sin1 = a.sin();
    println!("{:x}", sin1.to_bits());
    let sin2 = a.sin();
    println!("{:x}", sin2.to_bits());

    println!("{}", a);
}
