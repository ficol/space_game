use server;

#[test]
fn non_existing_path() {
    let result = server::run("non existing file", "0.0.0.0:8888");
    assert!(result.is_err());
}
