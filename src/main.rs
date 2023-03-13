fn main() {
    let a = f32::from_bits(1061273700);
    println!("{}", a);
    println!();

    let sin = a.sin();
    println!("sin\t\t{:x}", sin.to_bits());
    let cos = a.cos();
    println!("cos\t\t{:x}", cos.to_bits());

    let sincos_tuple1 = (sin, cos);
    println!("tuple1.sin\t{:x}", sincos_tuple1.0.to_bits());
    println!("tuple1.cos\t{:x}", sincos_tuple1.1.to_bits());
    let sincos_tuple2 = (a.sin(), a.cos());
    println!("tuple2.sin\t{:x}", sincos_tuple2.0.to_bits());
    println!("tuple2.cos\t{:x}", sincos_tuple2.1.to_bits());
    let sincos = a.sin_cos();
    println!("sincos.sin\t{:x}", sincos.0.to_bits());
    println!("sincos.cos\t{:x}", sincos.1.to_bits());
}
