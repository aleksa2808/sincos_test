fn main() {
    let a: f32 = bincode::deserialize(&[100, 192, 65, 63]).unwrap();
    println!("{:?}", a);

    let sin = a.sin();
    println!("sin {:?}", bincode::serialize(&sin).unwrap());
    let cos = a.cos();
    println!("cos {:?}", bincode::serialize(&cos).unwrap());

    let sincos_tuple1 = (sin, cos);
    println!(
        "sincos_tuple1 {:?}",
        bincode::serialize(&sincos_tuple1).unwrap()
    );
    println!(
        "sincos_tuple1.0 {:?}",
        bincode::serialize(&sincos_tuple1.0).unwrap()
    );
    println!(
        "sincos_tuple1.1 {:?}",
        bincode::serialize(&sincos_tuple1.1).unwrap()
    );

    let sincos_tuple2 = (a.sin(), a.cos());
    println!(
        "sincos_tuple2 {:?}",
        bincode::serialize(&sincos_tuple2).unwrap()
    );
    println!(
        "sincos_tuple2.0 {:?}",
        bincode::serialize(&sincos_tuple2.0).unwrap()
    );
    println!(
        "sincos_tuple2.1 {:?}",
        bincode::serialize(&sincos_tuple2.1).unwrap()
    );

    let sincos = a.sin_cos();
    println!("sincos {:?}", bincode::serialize(&sincos).unwrap());
    println!("sincos.0 {:?}", bincode::serialize(&sincos.0).unwrap());
    println!("sincos.1 {:?}", bincode::serialize(&sincos.1).unwrap());
}
