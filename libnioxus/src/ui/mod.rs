use dioxus::prelude::Props;

use self::spacing::Spacing;

pub mod spacing;

#[derive(derive_more::Display, Copy, Clone, Debug, Default)]
pub enum Display {
    #[default]
    Block,
    Inline,
    InlineBlock,
    #[display(fmt = "flex")]
    Flex,
    #[display(fmt = "inline-flex")]
    InlineFlex,
}

#[derive(Clone, Debug, Default)]
pub struct Padding {
    pub left: Option<Spacing>,
    pub right: Option<Spacing>,
    pub top: Option<Spacing>,
    pub bottom: Option<Spacing>,
}

impl Padding {
    pub fn all(padding: Spacing) -> Self {
        Self {
            left: Some(padding),
            right: Some(padding),
            top: Some(padding),
            bottom: Some(padding),
        }
    }

    pub fn x(padding_x: Spacing) -> Self {
        Self {
            left: Some(padding_x),
            right: Some(padding_x),
            ..Self::default()
        }
    }

    pub fn y(padding_y: Spacing) -> Self {
        Self {
            top: Some(padding_y),
            bottom: Some(padding_y),
            ..Self::default()
        }
    }

    pub fn with_x(self, padding_x: Spacing) -> Self {
        Self {
            left: Some(padding_x),
            right: Some(padding_x),
            ..self
        }
    }

    pub fn with_y(self, padding_y: Spacing) -> Self {
        Self {
            top: Some(padding_y),
            bottom: Some(padding_y),
            ..self
        }
    }
}

#[derive(Debug, Clone, Default, Props)]
pub struct TwProps {
    pub display: Option<Display>,
    pub padding: Option<Padding>,
}
