use ll::stylesheet::css_fixed;

pub fn css_fixed_to_float(f: css_fixed) -> float {
    const before: i32 = 10;
    f as float * 1.0f / ((1i32 << before) as float)
}
