mod types;

use bitpacker::Packable;

#[test]
fn test_packable() {
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

    let origin = types::F {
        x: types::a::A(1),
        y: types::a::A(2),
    };
    let packed = origin.pack();
    let unpacked = types::F::unpack(packed);
    assert_eq!(origin, unpacked);
}

#[test]
fn test_size() {
    assert_eq!(types::B::SIZE, 0);
    assert_eq!(types::C::SIZE, 6);
    assert_eq!(types::D::SIZE, 11);
    assert_eq!(types::E::SIZE, 0);
    assert_eq!(types::F::<types::a::A>::SIZE, 6);
}
