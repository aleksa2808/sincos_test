fn main() {
    let a: f32 = 0.7568419;
    println!("{:?}", a);

    let sin = a.sin();
    println!("sin {:x}", sin.to_bits());
    let cos = a.cos();
    println!("cos {:x}", cos.to_bits());

    let sincos_tuple1 = (sin, cos);
    println!("sincos_tuple1.0 {:x}", sincos_tuple1.0.to_bits());
    println!("sincos_tuple1.1 {:x}", sincos_tuple1.1.to_bits());

    let sincos_tuple2 = (a.sin(), a.cos());
    println!("sincos_tuple2.1 {:x}", sincos_tuple2.0.to_bits());
    println!("sincos_tuple2.1 {:x}", sincos_tuple2.1.to_bits());

    let sincos = a.sin_cos();
    println!("sincos.0 {:x}", sincos.0.to_bits());
    println!("sincos.1 {:x}", sincos.1.to_bits());
}
