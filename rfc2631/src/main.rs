use rfc2631::Group;

fn main() {
    let group = Group::new();
    let a = group.generate_key_pair();
    let b = group.generate_key_pair();
    println!("{:?}", a.zz(b.public_key()));
    println!("{:?}", b.zz(a.public_key()));
}
