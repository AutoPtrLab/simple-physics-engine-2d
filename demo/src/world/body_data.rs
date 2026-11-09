// funcitons to unpack the data of the bodies
//the user data is a u64 , the first 32 bits represent the entity kind and the other

use slotmap::KeyData;

use crate::world::EntityKind;

pub fn unpack_body_data(data: u128) -> (EntityKind, KeyData) {
    let kind = (data >> 64) as u64;
    (
        EntityKind::from_u64(kind).expect("the index exceeds the number of entities"),
        KeyData::from_ffi(data as u64),
    )
}

pub fn to_body_data(kind: EntityKind, handle: u64) -> u128 {
    (EntityKind::to_u64(kind) as u128) << 64 | handle as u128
}
