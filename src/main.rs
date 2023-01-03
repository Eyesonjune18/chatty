mod sender;
mod receiver;

fn main() -> std::io::Result<()> {
    sender::start()?;
    // receiver::start()?;

    Ok(())
}
