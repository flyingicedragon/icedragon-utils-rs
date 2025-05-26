use icedragon_utils_rs::prelude::*;

#[test]
fn tap_usize() {
    let x = 2usize.tap(|y| println!("{}", y));
    assert_eq!(x, 2);
}
