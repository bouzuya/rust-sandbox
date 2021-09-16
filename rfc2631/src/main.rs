use rfc2631::T;

fn main() {
    let a = T::new();
    let b = T::new();
    println!("{}", a.zz(&b));
}
