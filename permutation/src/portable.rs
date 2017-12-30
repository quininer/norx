#[cfg(feature = "config_6461")]
mod rot_const {
    pub const R0: u32 =  8;
    pub const R1: u32 = 19;
    pub const R2: u32 = 40;
    pub const R3: u32 = 63;
}

use ::{ W, L };
use self::rot_const::*;


pub fn norx(state: &mut [W; 16]) {
    /// The full round
    fn f(state: &mut [W; 16]) {
        macro_rules! G {
            ( $a:expr, $b:expr, $c:expr, $d:expr ) => {
                let (a, b, c, d) = g($a, $b, $c, $d);
                $a = a; $b = b; $c = c; $d = d;
            }
        }

        // Column step
        G!(state[ 0], state[ 4], state[ 8], state[12]);
        G!(state[ 1], state[ 5], state[ 9], state[13]);
        G!(state[ 2], state[ 6], state[10], state[14]);
        G!(state[ 3], state[ 7], state[11], state[15]);
        // Diagonal step
        G!(state[ 0], state[ 5], state[10], state[15]);
        G!(state[ 1], state[ 6], state[11], state[12]);
        G!(state[ 2], state[ 7], state[ 8], state[13]);
        G!(state[ 3], state[ 4], state[ 9], state[14]);
    }

    for _ in 0..L {
        f(state);
    }
}


/// The nonlinear primitive
#[inline]
fn h(a: W, b: W) -> W {
    (a ^ b) ^ ((a & b) << 1)
}

/// The quarter-round
#[inline]
fn g(mut a: W, mut b: W, mut c: W, mut d: W) -> (W, W, W, W) {
    a = h(a, b); d ^= a; d = d.rotate_right(R0);
    c = h(c, d); b ^= c; b = b.rotate_right(R1);
    a = h(a, b); d ^= a; d = d.rotate_right(R2);
    c = h(c, d); b ^= c; b = b.rotate_right(R3);
    (a, b, c, d)
}
