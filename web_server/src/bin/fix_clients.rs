use tnf_web_server::*;

fn main() -> std::io::Result<()> {
    let clients = env::expect_clients();
    env::setup_working_dir()?;

    let not_test = std::env::args().nth(1).as_ref().map(AsRef::as_ref) == Some("--notest");
    critters_db::CrittersDb::fix_clients(clients, !not_test);

    Ok(())
}