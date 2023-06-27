use self::units::CSSUnit;

pub mod units;

#[derive(Clone, Debug, PartialEq, derive_more::Display)]
pub enum CSSValue {
    #[display(fmt = "{_0}")]
    Unit(CSSUnit),
    #[display(fmt = "\"{_0}\"")]
    String(String),
    #[display(fmt = "{_0}")]
    Raw(String),
}
