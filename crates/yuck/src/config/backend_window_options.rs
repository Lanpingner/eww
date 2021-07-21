use std::str::FromStr;

use anyhow::*;

use crate::{
    enum_parse,
    error::AstResult,
    parser::{
        ast::{Ast, Span},
        ast_iterator::AstIterator,
        from_ast::FromAstElementContent,
    },
    value::NumWithUnit,
};

use super::{attributes::Attributes, window_definition::EnumParseError};

pub type BackendWindowOptions = X11WindowOptions;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct X11WindowOptions {
    pub wm_ignore: bool,
    pub sticky: bool,
    pub window_type: WindowType,
    pub struts: StrutDefinition,
}

impl X11WindowOptions {
    pub fn from_attrs(attrs: &mut Attributes) -> AstResult<Self> {
        let struts = attrs.ast_optional("reserve")?;
        let window_type = attrs.primitive_optional("windowtype")?;
        Ok(X11WindowOptions {
            wm_ignore: attrs.primitive_optional("wm-ignore")?.unwrap_or(window_type.is_none() && struts.is_none()),
            window_type: window_type.unwrap_or_default(),
            sticky: attrs.primitive_optional("sticky")?.unwrap_or(true),
            struts: struts.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, smart_default::SmartDefault, serde::Serialize)]
pub enum WindowType {
    #[default]
    Dock,
    Dialog,
    Toolbar,
    Normal,
    Utility,
}
impl FromStr for WindowType {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        enum_parse! { "window type", s,
            "dock" => Self::Dock,
            "toolbar" => Self::Toolbar,
            "dialog" => Self::Dialog,
            "normal" => Self::Normal,
            "utility" => Self::Utility,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, smart_default::SmartDefault, serde::Serialize)]
pub enum Side {
    #[default]
    Top,
    Left,
    Right,
    Bottom,
}

impl std::str::FromStr for Side {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Side, Self::Err> {
        enum_parse! { "side", s,
            "l" | "left" => Side::Left,
            "r" | "right" => Side::Right,
            "t" | "top" => Side::Top,
            "b" | "bottom" => Side::Bottom,
        }
    }
}

// Surface definition if the backend for X11 is enable
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, serde::Serialize)]
pub struct StrutDefinition {
    pub side: Side,
    pub dist: NumWithUnit,
}

impl FromAstElementContent for StrutDefinition {
    fn get_element_name() -> &'static str {
        "struts"
    }

    fn from_tail<I: Iterator<Item = Ast>>(span: Span, mut iter: AstIterator<I>) -> AstResult<Self> {
        let mut attrs = iter.expect_key_values()?;
        Ok(StrutDefinition { side: attrs.primitive_required("side")?, dist: attrs.primitive_required("distance")? })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WaylandWindowOptions {
    pub exclusive: bool,
    pub focusable: bool,
}
impl WaylandWindowOptions {
    pub fn from_attrs(attrs: &mut Attributes) -> AstResult<Self> {
        Ok(WaylandWindowOptions {
            exclusive: attrs.primitive_optional("exclusive")?.unwrap_or(false),
            focusable: attrs.primitive_optional("focusable")?.unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NoBackendWindowOptions;
impl NoBackendWindowOptions {
    pub fn from_attrs(attrs: &mut Attributes) -> Result<Self> {
        Ok(NoBackendWindowOptions)
    }
}
