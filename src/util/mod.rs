//pub mod util;

use std::f32;
use std::cmp;
use std::cmp::Ordering;

pub fn minf<T: cmp::PartialOrd>(v1: T, v2: T) -> T{
    let some_order = v1.partial_cmp(&v2);
    if let Some(ord) = some_order {
        let min_val = match ord {
            Ordering::Less => v1,
            _ => v2
        };
        return min_val;
    } else {
        panic!("Paniced comparing floats, one value is NaN");
    }
}

pub fn maxf<T: cmp::PartialOrd>(v1: T, v2: T) -> T{
    let some_order = v1.partial_cmp(&v2);
    if let Some(ord) = some_order {
        let max_val = match ord {
            Ordering::Less => v2,
            _ => v1
        };
        return max_val;
    } else {
        panic!("Paniced comparing floats, one value is NaN");
    }
}

#[test]
fn mintest() {
    let result = minf(13.3f32, 13.4f32);
    assert_eq!(result, 13.3f32);

    let result = minf(25f32, 0.3f32);
    assert_eq!(result, 0.3f32);
}
#[test]
fn maxtest() {
    let result = maxf(13.3f32, 13.4f32);
    assert_eq!(result, 13.4f32);

    let result = maxf(25f32, 0.3f32);
    assert_eq!(result, 25f32);
}
#[test]
#[should_panic]
fn mintest_panic(){
    let result = minf(25f32, f32::NAN);
}

