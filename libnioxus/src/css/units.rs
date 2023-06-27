#[derive(Copy, Clone, Debug, PartialEq, derive_more::Display)]
pub enum CSSUnit {
    // #region spacing
    /// pixels - `px`
    #[display(fmt = "{_0}px")]
    Px(f32),
    /// `em`
    #[display(fmt = "{_0}em")]
    Em(f32),
    /// `rem`
    #[display(fmt = "{_0}rem")]
    Rem(f32),
    /// viewport width - `vw`
    #[display(fmt = "{_0}vw")]
    ViewportWidth(f32),
    /// viewport height - `vh`
    #[display(fmt = "{_0}vh")]
    ViewportHeight(f32),
    /// smaller (the one with UI elements on mobile browsers) viewport width - `svw`
    #[display(fmt = "{_0}svw")]
    SmallerViewportWidth(f32),
    /// larger (the one without mobile browsers' UI elements) viewport width - `lvw`
    #[display(fmt = "{_0}lvw")]
    LargerViewportWidth(f32),
    // #endregion
}
