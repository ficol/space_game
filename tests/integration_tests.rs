use space_game;

#[test]
fn non_existing_path() {
    let result = space_game::run("non existing file", 0);
    assert!(result.is_err());
}
