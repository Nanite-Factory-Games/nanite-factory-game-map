use bevy::prelude::*;
use lightyear::prelude::Diffable;
use serde::{Deserialize, Serialize};

const MAX_POSITION_DELTA: f32 = 8.0;

#[derive(Component)]
pub struct ResourceEntity {
    pub id: u64
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct CharacterId(pub u16, pub u8);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CharacterPosition(pub u32, pub u32);

// Since between two ticks the position doesn't change much, we could encode
// the diff using a discrete set of values to reduce the bandwidth
impl Diffable<u8> for CharacterPosition {
    fn base_value() -> Self {
        Self(0, 0)
    }

    fn diff(&self, new: &Self) -> u8 {
        let mut diff_x = new.0 as i32 - self.0 as i32;
        let mut diff_y = new.1 as i32 - self.1 as i32;

        // Clamp the diff to a discrete set of values
        // i.e i4::MIN = -8.0, i4::MAX = 7.0
        diff_x = diff_x.clamp(-(MAX_POSITION_DELTA as i32), MAX_POSITION_DELTA as i32);
        diff_y = diff_y.clamp(-(MAX_POSITION_DELTA as i32), MAX_POSITION_DELTA as i32);
        let scaled_x = diff_x as f32 / MAX_POSITION_DELTA * 7.0;
        let scaled_y = diff_y as f32 / MAX_POSITION_DELTA * 7.0;

        let x_i4 = encode_i4(scaled_x);
        let y_i4 = encode_i4(scaled_y);
        (x_i4 << 4) | y_i4
    }

    fn apply_diff(&mut self, delta: &u8) {
        trace!("Applying diff {:?} to {:?}", delta, self);
        let packed = *delta;
        let x_i4 = decode_i4(packed >> 4) as f32;
        let y_i4 = decode_i4(packed & 0x0F) as f32;
        let diff_x = (x_i4 / 7.0 * MAX_POSITION_DELTA).round() as i32;
        let diff_y = (y_i4 / 7.0 * MAX_POSITION_DELTA).round() as i32;
        let next_x = (self.0 as i32 + diff_x).max(0) as u32;
        let next_y = (self.1 as i32 + diff_y).max(0) as u32;
        self.0 = next_x;
        self.1 = next_y;
    }
}

// Encode the value as a 4 bit signed integer 
fn encode_i4(value: f32) -> u8 {
    let clamped = value.round().clamp(-8.0, 7.0) as i8;
    (clamped as u8) & 0x0F
}

// Decode the vale from a 4 bit signed integer into a 8 bit signed integer
fn decode_i4(value: u8) -> i8 {
    let nibble = value & 0x0F;
    if (nibble & 0x08) != 0 {
        (nibble as i8) - 16
    } else {
        nibble as i8
    }
}

/// Marker to allow selecting characters that belong to the player
#[derive(Component)]
pub struct PlayerCharacterMarker;