mod types;

use bitpacker::Packable;

#[test]
fn it_works() {
    let origin = types::D::unit();
    let packed = origin.pack();
    let unpacked = types::D::unpack(packed);
    assert_eq!(origin, unpacked);

    let origin = types::D::empty();
    let packed = origin.pack();
    let unpacked = types::D::unpack(packed);
    assert_eq!(origin, unpacked);

    let origin = types::D::tuple(1, 2, 3);
    let packed = origin.pack();
    let unpacked = types::D::unpack(packed);
    assert_eq!(origin, unpacked);

    let origin = types::D::named(4, 5);
    let packed = origin.pack();
    let unpacked = types::D::unpack(packed);
    assert_eq!(origin, unpacked);

    let origin = types::E::Unit;
    let packed = origin.pack();
    let unpacked = types::E::unpack(packed);
    assert_eq!(origin, unpacked);
}
