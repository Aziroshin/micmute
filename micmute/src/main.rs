use micmute_lib::error::{MiMuError, MiMuErrorKind, MiMuResult};

fn main() -> MiMuResult<()> {
    let error = MiMuError::new(
        MiMuErrorKind::Misc("Test error.")
    );
    Err(error)
}