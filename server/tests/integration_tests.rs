use server;

#[test]
fn non_existing_path() {
    let result = server::run("non existing file", 0);
    assert!(result.is_err());
}
