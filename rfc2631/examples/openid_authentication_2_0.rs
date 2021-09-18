use rfc2631::{Generator, Group, Modulus};

fn main() -> anyhow::Result<()> {
    let g_bytes = hex::decode("02")?;
    let p_bytes = hex::decode(concat!(
        "DCF93A0B883972EC0E19989AC5A2CE310E1D37717E8D9571BB7623731866E61E",
        "F75A2E27898B057F9891C2E27A639C3F29B60814581CD3B2CA3986D268370557",
        "7D45C2E7E52DC81C7A171876E5CEA74B1448BFDFAF18828EFD2519F14E45E382",
        "6634AF1949E5B535CC829A483B8A76223E5D490A257F05BDFF16F2FB22C583AB"
    ))?;
    let g = Generator::from_bytes_be(&g_bytes);
    let p = Modulus::from_bytes_be(&p_bytes);
    let group = Group::new(g, p);
    let a = group.generate_key_pair();
    let b = group.generate_key_pair();
    println!("{:?}", a.shared_secret(b.public_key()));
    println!("{:?}", b.shared_secret(a.public_key()));
    Ok(())
}
