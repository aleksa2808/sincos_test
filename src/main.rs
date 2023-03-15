fn main() {
    let a: f32 = bincode::deserialize(&[100, 192, 65, 63]).unwrap();
    println!("{:?}", a);
    println!("{:x}", a.to_bits());

    let sin = a.sin();
    println!("sin {:?}", bincode::serialize(&sin).unwrap());
    println!("sin {:b}", sin.to_bits());
    println!("sin {:x}", sin.to_bits());
    let cos = a.cos();
    println!("cos {:?}", bincode::serialize(&cos).unwrap());
    println!("cos {:b}", cos.to_bits());
    println!("cos {:x}", cos.to_bits());

    let sincos_tuple1 = (sin, cos);
    println!(
        "sincos_tuple1 {:?}",
        bincode::serialize(&sincos_tuple1).unwrap()
    );
    println!(
        "sincos_tuple1.0 {:?}",
        bincode::serialize(&sincos_tuple1.0).unwrap()
    );
    println!("sincos_tuple1.0 {:b}", sincos_tuple1.0.to_bits());
    println!("sincos_tuple1.0 {:x}", sincos_tuple1.0.to_bits());
    println!(
        "sincos_tuple1.1 {:?}",
        bincode::serialize(&sincos_tuple1.1).unwrap()
    );
    println!("sincos_tuple1.1 {:b}", sincos_tuple1.1.to_bits());
    println!("sincos_tuple1.1 {:x}", sincos_tuple1.1.to_bits());

    let sincos_tuple2 = (a.sin(), a.cos());
    println!(
        "sincos_tuple2 {:?}",
        bincode::serialize(&sincos_tuple2).unwrap()
    );
    println!(
        "sincos_tuple2.0 {:?}",
        bincode::serialize(&sincos_tuple2.0).unwrap()
    );
    println!("sincos_tuple2.1 {:b}", sincos_tuple2.0.to_bits());
    println!("sincos_tuple2.1 {:x}", sincos_tuple2.0.to_bits());
    println!(
        "sincos_tuple2.1 {:?}",
        bincode::serialize(&sincos_tuple2.1).unwrap()
    );
    println!("sincos_tuple2.1 {:b}", sincos_tuple2.1.to_bits());
    println!("sincos_tuple2.1 {:x}", sincos_tuple2.1.to_bits());

    let sincos = a.sin_cos();
    println!("sincos {:?}", bincode::serialize(&sincos).unwrap());
    println!("sincos.0 {:?}", bincode::serialize(&sincos.0).unwrap());
    println!("sincos.0 {:b}", sincos.0.to_bits());
    println!("sincos.0 {:x}", sincos.0.to_bits());
    println!("sincos.1 {:?}", bincode::serialize(&sincos.1).unwrap());
    println!("sincos.1 {:b}", sincos.1.to_bits());
    println!("sincos.1 {:x}", sincos.1.to_bits());
}
