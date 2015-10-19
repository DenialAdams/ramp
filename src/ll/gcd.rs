use std::cmp::Ordering;

use ll;
use ll::limb::Limb;

pub unsafe fn gcd(mut gp: *mut Limb, mut ap: *mut Limb, mut an: i32, mut bp: *mut Limb, mut bn: i32) -> i32 {
    assert!(an >= bn);

    let mut gc = 0;
    while *ap == 0 && !ll::is_zero(ap, an) && *bp == 0 && !ll::is_zero(bp, bn){
        ap = ap.offset(1);
        bp = bp.offset(1);
        gp = gp.offset(1);
        an -= 1;
        bn -= 1;
        gc += 1;
    }

    let a_trailing = (*ap).trailing_zeros() as u32;
    let b_trailing = (*bp).trailing_zeros() as u32;

    let trailing = if a_trailing <= b_trailing {
        a_trailing
    } else {
        b_trailing
    };
    if trailing > 0 {
        ll::shr(ap, ap, an, trailing);
        ll::shr(bp, bp, bn, trailing);
    }

    let mut ac : usize = trailing as usize;
    let mut bc : usize = trailing as usize;
    while an > 0 && !ll::is_zero(ap, an) {
        while *ap == 0 && !ll::is_zero(ap, an) {
            ap = ap.offset(1);
            an -= 1;
            // ac = 0;
        }
        if ll::is_zero(ap, an) {
            break;
        }

        let at = (*ap).trailing_zeros() as u32;
        if at > 0 {
            ll::shr(ap, ap, an, at);
            ac += at as usize;
            if ac > Limb::BITS {
                an -= 1;
                ac = ac % (Limb::BITS  + 1);
            }
        }

        while *bp == 0 && !ll::is_zero(bp, bn) {
            bp = bp.offset(1);
            bn -= 1;
            // bc = 0;
        }
        if ll::is_zero(bp, bn) {
            break;
        }

        let bt = (*bp).trailing_zeros() as u32;
        if bt > 0 {
            ll::shr(bp, bp, bn, bt);
            bc += bt as usize;
            if bc > Limb::BITS {
                bn -= 1;
                bc = bc % (Limb::BITS  + 1);
            }
        }

        let c = if an == bn {
            ll::cmp(ap, bp, an)
        } else if an > bn {
            Ordering::Greater
        } else {
            Ordering::Less
        };

        if c == Ordering::Equal || c == Ordering::Greater {
            ll::sub(ap, ap, an, bp, bn);
            ll::shr(ap, ap, an, 1);
        } else {
            ll::sub(bp, bp, bn, ap, an);
            ll::shr(bp, bp, bn, 1);
        }
    }

    ll::copy_incr(bp, gp, bn);
    if trailing > 0 {
        let v = ll::shl(gp, gp, bn, trailing);
        if v > 0 {
            *gp.offset(bn as isize) = v;
        }
    }

    gc + bn
}
