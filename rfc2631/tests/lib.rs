use num_bigint::BigUint;
use rfc2631::{Generator, Group, Modulus};

#[test]
fn bytes_debug_test() {
    // bytes not impl Display
    let bytes: &[u8] = &[0x00, 0x7F];
    assert_eq!("[0, 127]", format!("{:?}", bytes));
}

#[test]
fn test() -> anyhow::Result<()> {
    let g = Generator::from_bytes_be(&[0x02]);
    let p = Modulus::from_bytes_be(
        &BigUint::parse_bytes(
            concat!(
                "DCF93A0B883972EC0E19989AC5A2CE310E1D37717E8D9571BB7623731866E61E",
                "F75A2E27898B057F9891C2E27A639C3F29B60814581CD3B2CA3986D268370557",
                "7D45C2E7E52DC81C7A171876E5CEA74B1448BFDFAF18828EFD2519F14E45E382",
                "6634AF1949E5B535CC829A483B8A76223E5D490A257F05BDFF16F2FB22C583AB"
            )
            .as_bytes(),
            16,
        )
        .unwrap()
        .to_bytes_be(),
    );
    let group = Group::new(g, p);
    let a = group.generate_key_pair();
    let b = group.generate_key_pair();
    assert_eq!(
        a.shared_secret(b.public_key()),
        b.shared_secret(a.public_key())
    );

    let a2 = group.create_key_pair_from_private_key(a.private_key().clone())?;
    assert_eq!(a, a2);

    Ok(())
}
