use rfc2631::T;

fn main() {
    let a = T::generate_x();
    let b = T::generate_x();
    println!("{:?}", a.zz(&b));
}
