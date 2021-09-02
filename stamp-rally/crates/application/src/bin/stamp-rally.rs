use adapter_console::run;
use use_case::CreateStampRally;

fn main() -> anyhow::Result<()> {
    let create_stamp_rally_use_case = CreateStampRally::new();
    run(create_stamp_rally_use_case)
}
