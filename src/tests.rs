use crate::JLSTZ;

#[test]
fn test_jlstz() {
    let mut l = JLSTZ::new([
        0, 0, 1,
        1, 1, 1,
        0, 0, 0
    ]);

    l.rotate();
    assert_eq!(l.block_pos, [0, 1, 0, 0, 1, 0, 0, 1, 1]);
    l.rotate();
    assert_eq!(l.block_pos, [0, 0, 0, 1, 1, 1, 1, 0, 0]);
    l.rotate();
    assert_eq!(l.block_pos, [1, 1, 0, 0, 1, 0, 0, 1, 0]);
    l.rotate();
    assert_eq!(l.block_pos, [0, 0, 1, 1, 1, 1, 0, 0, 0]);
    
    let mut j = JLSTZ::new([
        1, 0, 0,
        1, 1, 1,
        0, 0, 0
    ]);

    j.rotate();
    assert_eq!(j.block_pos, [0, 1, 1, 0, 1, 0, 0, 1, 0]);
    j.rotate();
    assert_eq!(j.block_pos, [0, 0, 0, 1, 1, 1, 0, 0, 1]);
    j.rotate();
    assert_eq!(j.block_pos, [0, 1, 0, 0, 1, 0, 1, 1, 0]);
    j.rotate();
    assert_eq!(j.block_pos, [1, 0, 0, 1, 1, 1, 0, 0, 0]);

    let mut s = JLSTZ::new([
        0, 1, 1,
        1, 1, 0,
        0, 0, 0
    ]);

    s.rotate();
    assert_eq!(s.block_pos, [0, 1, 0, 0, 1, 1, 0, 0, 1]);

    let mut t = JLSTZ::new([
        0, 1, 0,
        1, 1, 1,
        0, 0, 0
    ]);

    t.rotate();
    assert_eq!(t.block_pos, [0, 1, 0, 0, 1, 1, 0, 1, 0]);

    let mut z = JLSTZ::new([
        1, 1, 0,
        0, 1, 1,
        0, 0, 0
    ]);

    z.rotate();
    assert_eq!(z.block_pos, [0, 0, 1, 0, 1, 1, 0, 1, 0]);

}