use vector_lib::*;

pub fn main() {
    let u = Vector::new(2.0, 4.0, 6.0);
    let v = Vector::new(3.0, 4.0, 5.0);

    println!("Dot Product: {}", u.dot_product(&v));

    println!("Cross Product (u x v): {}", u.cross_product(&v));
    println!("Cross Product (v x u): {}", v.cross_product(&u));

    println!("u Scaled to 2.0x: {}", &u * 2.0);
    println!("u Scaled to 1/2: {}", &u * (1.0 / 2.0));

    println!("u projected onto v: {}", u.proj_onto(&v));
    println!("v projected onto u: {}", v.proj_onto(&u));
}
