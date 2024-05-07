use crate::Float;

use super::fma;

pub struct CompensatedFloat {
    v: Float,
    err: Float,
}

impl CompensatedFloat {
    pub fn to_float(&self) -> Float {
        self.v + self.err
    }
}

#[inline]
fn two_prod(a: Float, b: Float) -> CompensatedFloat {
    let ab = a * b;
    CompensatedFloat {
        v: ab,
        err: fma(a, b, -ab),
    }
}

#[inline]
fn two_sum(a: Float, b: Float) -> CompensatedFloat {
    let s = a + b;
    let delta = s - a;
    CompensatedFloat {
        v: s,
        err: (a - (s - delta)) + (b - delta),
    }
}

#[inline]
pub fn inner_product_internal_2(a: Float, b: Float) -> CompensatedFloat {
    two_prod(a, b)
}

#[inline]
pub fn inner_product_internal_4(a: Float, b: Float, c: Float, d: Float) -> CompensatedFloat {
    let ab = two_prod(a, b);
    let tp = inner_product_internal_2(c, d);
    let sum = two_sum(ab.v, tp.v);
    CompensatedFloat {
        v: sum.v,
        err: ab.err + (tp.err + sum.err),
    }
}

#[inline]
pub fn inner_product_internal_6(
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    e: Float,
    f: Float,
) -> CompensatedFloat {
    let ab = two_prod(e, f);
    let tp = inner_product_internal_4(a, b, c, d);
    let sum = two_sum(ab.v, tp.v);
    CompensatedFloat {
        v: sum.v,
        err: ab.err + (tp.err + sum.err),
    }
}

#[inline]
pub fn inner_product_internal_8(
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    e: Float,
    f: Float,
    g: Float,
    h: Float,
) -> CompensatedFloat {
    let ab = two_prod(g, h);
    let tp = inner_product_internal_6(a, b, c, d, e, f);
    let sum = two_sum(ab.v, tp.v);
    CompensatedFloat {
        v: sum.v,
        err: ab.err + (tp.err + sum.err),
    }
}

#[inline]
pub fn inner_product_internal_10(
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    e: Float,
    f: Float,
    g: Float,
    h: Float,
    i: Float,
    j: Float,
) -> CompensatedFloat {
    let ab = two_prod(i, j);
    let tp = inner_product_internal_8(a, b, c, d, e, f, g, h);
    let sum = two_sum(ab.v, tp.v);
    CompensatedFloat {
        v: sum.v,
        err: ab.err + (tp.err + sum.err),
    }
}

#[inline]
pub fn inner_product_internal_12(
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    e: Float,
    f: Float,
    g: Float,
    h: Float,
    i: Float,
    j: Float,
    k: Float,
    l: Float,
) -> CompensatedFloat {
    let ab = two_prod(k, l);
    let tp = inner_product_internal_10(a, b, c, d, e, f, g, h, i, j);
    let sum = two_sum(ab.v, tp.v);
    CompensatedFloat {
        v: sum.v,
        err: ab.err + (tp.err + sum.err),
    }
}
