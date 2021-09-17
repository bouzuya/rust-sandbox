use rfc2631::Group;

fn main() {
    let group = Group::new();
    let a = group.generate_key_pair();
    let b = group.generate_key_pair();
    println!("{:?}", a.shared_secret(b.public_key()));
    println!("{:?}", b.shared_secret(a.public_key()));
}
