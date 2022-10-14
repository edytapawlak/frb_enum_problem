#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.49.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;

// Section: imports

// Section: wire functions

fn wire_generate_prefixed_text_impl(
    port_: MessagePort,
    strategy: impl Wire2Api<PrefixAlias> + UnwindSafe,
    basic_text: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "generate_prefixed_text",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_strategy = strategy.wire2api();
            let api_basic_text = basic_text.wire2api();
            move |task_callback| {
                Ok(mirror_PrefixedTextAlias(generate_prefixed_text(
                    api_strategy,
                    api_basic_text,
                )))
            }
        },
    )
}
fn wire_generate_sufixed_text_impl(
    port_: MessagePort,
    strategy: impl Wire2Api<SufixAlias> + UnwindSafe,
    basic_text: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "generate_sufixed_text",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_strategy = strategy.wire2api();
            let api_basic_text = basic_text.wire2api();
            move |task_callback| {
                Ok(mirror_SufixedTextAlias(generate_sufixed_text(
                    api_strategy,
                    api_basic_text,
                )))
            }
        },
    )
}
fn wire_transformed_text_from_str_impl(
    port_: MessagePort,
    str: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "transformed_text_from_str",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_str = str.wire2api();
            move |task_callback| Ok(mirror_TransformedText(transformed_text_from_str(api_str)))
        },
    )
}
// Section: wrapper structs

#[derive(Clone)]
struct mirror_PrefixAlias(PrefixAlias);

#[derive(Clone)]
struct mirror_PrefixedTextAlias(PrefixedTextAlias);

#[derive(Clone)]
struct mirror_SufixAlias(SufixAlias);

#[derive(Clone)]
struct mirror_SufixedTextAlias(SufixedTextAlias);

#[derive(Clone)]
struct mirror_TransformedText(TransformedText);

// Section: static checks

const _: fn() = || {
    match None::<PrefixAlias>.unwrap() {
        PrefixAlias::A => {}
        PrefixAlias::B => {}
        PrefixAlias::C => {}
    }
    {
        let PrefixedTextAlias = None::<PrefixedTextAlias>.unwrap();
        let _: PrefixAlias = PrefixedTextAlias.prefix;
        let _: String = PrefixedTextAlias.text;
    }
    match None::<SufixAlias>.unwrap() {
        SufixAlias::A => {}
        SufixAlias::B => {}
        SufixAlias::C => {}
    }
    {
        let SufixedTextAlias = None::<SufixedTextAlias>.unwrap();
        let _: SufixAlias = SufixedTextAlias.sufix;
        let _: String = SufixedTextAlias.text;
    }
    match None::<TransformedText>.unwrap() {
        TransformedText::Sufixed(field0) => {
            let _: SufixedTextAlias = field0;
        }
        TransformedText::Prefixed(field0) => {
            let _: PrefixedTextAlias = field0;
        }
    }
};
// Section: allocate functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<PrefixAlias> for i32 {
    fn wire2api(self) -> PrefixAlias {
        match self {
            0 => PrefixAlias::A,
            1 => PrefixAlias::B,
            2 => PrefixAlias::C,
            _ => unreachable!("Invalid variant for PrefixAlias: {}", self),
        }
    }
}
impl Wire2Api<SufixAlias> for i32 {
    fn wire2api(self) -> SufixAlias {
        match self {
            0 => SufixAlias::A,
            1 => SufixAlias::B,
            2 => SufixAlias::C,
            _ => unreachable!("Invalid variant for SufixAlias: {}", self),
        }
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for mirror_PrefixAlias {
    fn into_dart(self) -> support::DartAbi {
        match self.0 {
            PrefixAlias::A => 0,
            PrefixAlias::B => 1,
            PrefixAlias::C => 2,
        }
        .into_dart()
    }
}
impl support::IntoDart for mirror_PrefixedTextAlias {
    fn into_dart(self) -> support::DartAbi {
        vec![
            mirror_PrefixAlias(self.0.prefix).into_dart(),
            self.0.text.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_PrefixedTextAlias {}

impl support::IntoDart for mirror_SufixAlias {
    fn into_dart(self) -> support::DartAbi {
        match self.0 {
            SufixAlias::A => 0,
            SufixAlias::B => 1,
            SufixAlias::C => 2,
        }
        .into_dart()
    }
}
impl support::IntoDart for mirror_SufixedTextAlias {
    fn into_dart(self) -> support::DartAbi {
        vec![
            mirror_SufixAlias(self.0.sufix).into_dart(),
            self.0.text.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_SufixedTextAlias {}

impl support::IntoDart for mirror_TransformedText {
    fn into_dart(self) -> support::DartAbi {
        match self.0 {
            TransformedText::Sufixed(field0) => vec![0.into_dart(), field0.into_dart()],
            TransformedText::Prefixed(field0) => vec![1.into_dart(), field0.into_dart()],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_TransformedText {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
