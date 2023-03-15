fn main() {
    let a: f32 = 0.7568419;
    format!("{}", a);

    let tuple = (a.sin(), a.cos());
    println!("tuple.sin {:x}", tuple.0.to_bits());
    format!("{}", tuple.1);

    let sincos = a.sin_cos();
    println!("sincos.sin {:x}", sincos.0.to_bits());
    format!("{}", sincos.1);
}
