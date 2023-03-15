fn main() {
    let a: f32 = 0.7568419;

    let tuple = (a.sin(), a.cos());
    println!("tuple.1 {:x}", tuple.0.to_bits());
    println!("tuple.1 {:x}", tuple.1.to_bits());

    let sincos = a.sin_cos();
    println!("sincos.0 {:x}", sincos.0.to_bits());
    println!("sincos.1 {:x}", sincos.1.to_bits());
}
