use micmute_lib::error::{MiMuError, MiMuErrorKind, MiMuResult};

fn test2() -> MiMuResult<()> {
    let e = MiMuError::new(MiMuErrorKind::Misc,
        ""
    );
    Err(e)
}

fn test() -> MiMuResult<()> {
    test2()?;
    Ok(())
}

// Using MiMuResult directly results in no-region-bound-pairs for HirId.
// fn main() -> MiMuResult<()> {
//     test()?;
//     Ok(())
// }

// Same problem using prelude's Result.
fn main() -> Result<(), MiMuError<MiMuErrorKind>> {
    test()?;
    Ok(())
}