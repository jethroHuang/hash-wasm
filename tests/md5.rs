use std::fs;

use hash_wasm::hashs::Md5Hasher;

#[test]
#[ignore = "This test is ignored because it is too slow"]
fn test_update() {
    let mut md5 = Md5Hasher::new();
    let file = fs::read("/Users/jethro/Desktop/zh-cn_windows_11_consumer_editions_version_23h2_x64_dvd_91207780.iso").unwrap();
    file.chunks(524288000).for_each(|chunk| {
        md5.update(chunk);
    });
    assert_eq!(md5.digest(), "071620252ec577d55c67f00a4db039f1");
}