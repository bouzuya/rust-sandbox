use assert_cmd::Command;

#[test]
fn main() -> anyhow::Result<()> {
    let input = "https://www.amazon.co.jp/dp/4873118174/?coliid=I1IVE2TUH5XLB6&colid=112OTYINW3M1V&psc=1&ref_=lv_ov_lig_dp_it";
    let output = "https://www.amazon.co.jp/dp/4873118174\n";
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    command.arg(input).assert().success().stdout(output);
    Ok(())
}
