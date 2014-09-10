use piston::AssetStore;

use super::GameConfig;

#[test]
fn reads_a_json_config_file_into_GameConfig() {
    let testfiles = AssetStore::from_folder("../src/config/testfiles");
    let test_file = testfiles.path("settings.json").unwrap();
    let config = GameConfig::new(test_file);

    assert_eq!(config.window.width, 1024)
    assert_eq!(config.window.height, 768)
    assert!(config.window.fullscreen)

    assert_eq!(config.input.len(), 2);
    assert_eq!(config.input.get(&String::from_str("MoveForward")), &String::from_str("E"));
    assert_eq!(config.input.get(&String::from_str("MoveBackward")), &String::from_str("D"));
}

#[test]
#[should_fail]
fn errors_if_config_file_does_not_exist() {
    let testfiles = AssetStore::from_folder("../src/config/testfiles");
    let test_file = testfiles.path("not_there.json").unwrap();
    let config = GameConfig::new(test_file);
}

#[test]
#[should_fail]
fn errors_if_json_does_not_conform() {
    let testfiles = AssetStore::from_folder("../src/config/testfiles");
    let test_file = testfiles.path("bad.json").unwrap();
    let config = GameConfig::new(test_file);
}
