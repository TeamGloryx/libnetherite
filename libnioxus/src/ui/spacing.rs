use nmacro::im_spacing;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Spacing {
    Rem(f32),
    Em(f32),
    Px(f32),
}

im_spacing! {
    0 => 0,
    0.5 => 0.125,
    1 => 0.25,
    1.5 => 0.375,
    2 => 0.5,
    2.5 => 0.625,
    3 => 0.75,
    3.5 => 0.875,
    4 => 1.0,
    5 => 1.25,
    6 => 1.5,
    7 => 1.75,
    8 => 2.0,
    9 => 2.25,
    10 => 2.5,
    11 => 2.75,
    12 => 3.0,
    14 => 3.5,
    16 => 4.0,
    20 => 5.0,
    24 => 6.0,
    28 => 7.0,
    32 => 8.0,
    36 => 9.0,
    40 => 10.0,
    44 => 11.0,
    48 => 12.0,
    52 => 13.0,
    56 => 14.0,
    60 => 15.0,
    64 => 16.0,
    72 => 18.0,
    80 => 20.0,
    96 => 24.0
}

#[cfg(test)]
mod tests {
    use super::{tw_spacing, Spacing};

    #[test]
    fn test_tailwind_spacing() {
        assert_eq!(tw_spacing![4], Spacing::Rem(1.0f32));
        assert_eq!(tw_spacing![6], Spacing::Rem(1.5f32));
        assert_eq!(tw_spacing![8], Spacing::Rem(2.0f32));
    }
}
