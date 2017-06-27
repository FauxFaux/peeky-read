use std::io::{Cursor, Read};
use ::PeekyRead;

#[test]
fn zero() {
    let real = [];
    let mut inner = Cursor::new(real);
    let mut reader = PeekyRead::new(&mut inner);
    assert!(reader.check_eof().unwrap());
    assert!(reader.check_eof().unwrap());

    let mut buf = [0u8; 1];
    assert_eq!(0, reader.read(&mut buf).unwrap());
}

#[test]
fn one() {
    let real = [7u8];
    let mut inner = Cursor::new(real);
    let mut reader = PeekyRead::new(&mut inner);
    assert_eq!(false, reader.check_eof().unwrap());
    assert_eq!(false, reader.check_eof().unwrap());

    let mut buf = [0u8; 4096];
    assert_eq!(1, reader.read(&mut buf).unwrap());
    assert_eq!(7u8, buf[0]);

    assert!(reader.check_eof().unwrap());
    assert!(reader.check_eof().unwrap());

    assert_eq!(0, reader.read(&mut buf).unwrap());
}

#[test]
fn multi() {
    let real = [7u8, 8, 9, 10];
    let mut inner = Cursor::new(real);
    let mut reader = PeekyRead::new(&mut inner);
    assert_eq!(false, reader.check_eof().unwrap());

    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf).unwrap();
    assert_eq!(real, buf);

    assert!(reader.check_eof().unwrap());
    assert!(reader.check_eof().unwrap());

    assert_eq!(0, reader.read(&mut buf).unwrap());
}

#[test]
fn empty_buf() {
    let real = [7u8, 8, 9, 10];
    let mut inner = Cursor::new(real);
    let mut reader = PeekyRead::new(&mut inner);
    let mut buf = [];

    assert_eq!(0, reader.read(&mut buf).expect("empty read"));
    assert_eq!(false, reader.check_eof().unwrap());
    assert_eq!(0, reader.read(&mut buf).expect("empty read"));
}
