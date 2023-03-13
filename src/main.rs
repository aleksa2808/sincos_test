fn main() {
    let a: f32 = 0.7568419;

    assert_eq!(
        format!("{:x}", a.sin().to_bits()),
        format!("{:x}", a.sin().to_bits())
    );
    println!("{a}");
}
