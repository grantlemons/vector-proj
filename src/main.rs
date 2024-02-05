use vector_lib::*;

pub fn main() {
    let u = Vector::new(-8_f32, -2_f32, 20_f32);
    let v = Vector::new(-6_f32, -12_f32, 36_f32);

    println!("Dot Product: {}", u.dot_product(&v));

    println!("Cross Product (u x v): {}", u.cross_product(&v));
    println!("Cross Product (v x u): {}", v.cross_product(&u));

    println!("u Scaled to 2x: {}", &u * 2_f32);
    println!("u Scaled to 1/2x: {}", &u * (1_f32 / 2_f32));

    println!("u projected onto v: {}", u.proj_onto(&v));
    println!("v projected onto u: {}", v.proj_onto(&u));
}
