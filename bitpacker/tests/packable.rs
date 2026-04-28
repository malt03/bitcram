mod types;

use bitpacker::{Buffer, Packable};
use types::*;

fn assert_round_trip<P, B>(value: P)
where
    P: Packable<B> + std::fmt::Debug + PartialEq,
    B: Buffer,
{
    let packed = value.pack();
    let unpacked = P::unpack(packed);
    assert_eq!(value, unpacked);
}

#[test]
fn struct_unit() {
    assert_eq!(UnitStruct::SIZE, 0);
    assert_round_trip(UnitStruct);
}

#[test]
fn struct_tuple() {
    assert_eq!(TupleStruct::SIZE, 6);
    assert_round_trip(TupleStruct(U3(1), U3(2), UnitStruct));
}

#[test]
fn struct_named() {
    assert_eq!(NamedStruct::SIZE, 6);
    assert_round_trip(NamedStruct { x: U3(3), y: U3(5) });
}

#[test]
fn enum_single_variant() {
    assert_eq!(SingleVariantEnum::SIZE, 0);
    assert_round_trip(SingleVariantEnum::Only);
}

#[test]
fn enum_mixed_variants() {
    // 4 variants → 2-bit index; max payload is Tuple(U3 + TupleStruct) = 9 bits
    assert_eq!(MixedEnum::SIZE, 11);

    assert_round_trip(MixedEnum::UnitVariant);
    assert_round_trip(MixedEnum::EmptyTuple());
    assert_round_trip(MixedEnum::Tuple(
        U3(1),
        TupleStruct(U3(2), U3(3), UnitStruct),
    ));
    assert_round_trip(MixedEnum::Named {
        x: UnitStruct,
        y: TupleStruct(U3(4), U3(5), UnitStruct),
    });
}

#[test]
fn generic_struct() {
    assert_eq!(GenericPair::<U3>::SIZE, 6);
    assert_round_trip(GenericPair {
        x: U3(1),
        y: U3(2),
    });
}

#[test]
fn buffer_boundary_u8() {
    assert_eq!(FullU8::SIZE, 8);
    assert_round_trip(FullU8 {
        hi: Nibble(0x0),
        lo: Nibble(0x0),
    });
    assert_round_trip(FullU8 {
        hi: Nibble(0xF),
        lo: Nibble(0xF),
    });
    assert_round_trip(FullU8 {
        hi: Nibble(0xA),
        lo: Nibble(0x5),
    });
}

#[test]
fn buffer_boundary_u128() {
    assert_eq!(FullU128::SIZE, 128);
    assert_round_trip(FullU128 {
        hi: U64v(0),
        lo: U64v(0),
    });
    assert_round_trip(FullU128 {
        hi: U64v(u64::MAX),
        lo: U64v(u64::MAX),
    });
    assert_round_trip(FullU128 {
        hi: U64v(0xDEAD_BEEF_CAFE_BABE),
        lo: U64v(0x0123_4567_89AB_CDEF),
    });
}
