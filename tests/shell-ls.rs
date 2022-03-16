use ezexec::{ ExecBuilder, error::Result };

#[test]
fn ls() -> Result {
    ExecBuilder::with_shell("ls")?
        .spawn_transparent()?
        .wait()?;
    Ok(())
}
