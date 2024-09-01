pub fn pixel_add(a: f64, b: f64) -> f64 {
    a + b
}
pub fn pixel_sub(a: f64, b: f64) -> f64 {
    a - b
}
pub fn pixel_div(a: f64, b: f64) -> f64 {
    if b == 0. {
        return 1.
    }
    a / b
}
pub fn pixel_darker(a: f64, b: f64) -> f64 {
    a.min(b)
}
pub fn pixel_lighter(a: f64, b: f64) -> f64 {
    a.max(b)
}
pub fn pixel_diff(a: f64, b: f64) -> f64 {
    (a - b).abs()
}
pub fn pixel_mult(a: f64, b: f64) -> f64 {
    a * b
}
pub fn pixel_screen(a: f64, b: f64) -> f64 {
    1.0 - (1.0 - a) * (1.0 - b)
}
pub fn pixel_overlay(a: f64, b: f64) -> f64 {
    if a < 0.5 {
        2.0 * a * b
    } else {
        1.0 - 2.0 * (1.0 - a) * (1.0 - b)
    }
}
pub fn pixel_hard_light(a: f64, b: f64) -> f64 {
    if b < 0.5 {
        2.0 * a * b
    } else {
        1.0 - 2.0 * (1.0 - a) * (1.0 - b)
    }
}
pub fn pixel_soft_light(a: f64, b: f64) -> f64 {
    if b <= 0.5 {
        a - (1.0 - 2.0 * b) * a * (1.0 - a)
    } else {
        let gwc3 = if a <= 0.25 {
            ((16.0 * a - 12.0) * a + 4.0) * a
        } else {
            a.sqrt()
        };
        a + (2.0 * b - 1.0) * (gwc3 - a)
    }
}
