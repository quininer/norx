use ::{ U, S, L };
use ::rot_const::*;


pub fn norx(state: &mut [U; S]) {
    /// The full round
    fn f(state: &mut [U; S]) {
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
fn h(a: U, b: U) -> U {
    (a ^ b) ^ ((a & b) << 1)
}

/// The quarter-round
#[inline]
fn g(mut a: U, mut b: U, mut c: U, mut d: U) -> (U, U, U, U) {
    a = h(a, b); d ^= a; d = d.rotate_right(R0);
    c = h(c, d); b ^= c; b = b.rotate_right(R1);
    a = h(a, b); d ^= a; d = d.rotate_right(R2);
    c = h(c, d); b ^= c; b = b.rotate_right(R3);
    (a, b, c, d)
}
