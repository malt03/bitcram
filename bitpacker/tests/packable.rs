use bitpacker::{Packable, packable};

struct A(u8);

impl Packable<u32> for A {
    const SIZE: u32 = 2;
    fn pack(&self) -> u32 {
        self.0 as u32
    }
    fn unpack(buffer: u32) -> Self {
        Self(buffer as u8)
    }
}

#[packable(u32)]
struct B(A, A);

#[packable(u32)]
struct C {
    x: A,
    y: B,
}

#[packable(u32)]
enum D {
    W(),
    X(A, B),
    Y { x: C, y: A },
    Z,
}

#[test]
fn it_works() {
    let d = D::Y {
        x: C {
            x: A(1),
            y: B(A(2), A(3)),
        },
        y: A(4),
    };
    let packed = d.pack();
    eprintln!("packed: {packed}");
    // let bar = B(A(1), A(2));
    // let packed = bar.pack();
    // assert_eq!(packed, 1 << 2 | 2);
}
