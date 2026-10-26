use void::{ResultVoidErrExt, ResultVoidExt, Void};

#[test]
fn void_unwrap_ok() {
    let r: Result<u32, Void> = Ok(5);
    assert_eq!(5, r.void_unwrap());
}

#[test]
fn void_unwrap_err_err() {
    let r: Result<Void, u32> = Err(7);
    assert_eq!(7, r.void_unwrap_err());
}
