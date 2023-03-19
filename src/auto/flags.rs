// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use std::fmt;

bitflags! {
    #[doc(alias = "JSCValuePropertyFlags")]
    pub struct ValuePropertyFlags: u32 {
        #[doc(alias = "JSC_VALUE_PROPERTY_CONFIGURABLE")]
        const CONFIGURABLE = ffi::JSC_VALUE_PROPERTY_CONFIGURABLE as _;
        #[doc(alias = "JSC_VALUE_PROPERTY_ENUMERABLE")]
        const ENUMERABLE = ffi::JSC_VALUE_PROPERTY_ENUMERABLE as _;
        #[doc(alias = "JSC_VALUE_PROPERTY_WRITABLE")]
        const WRITABLE = ffi::JSC_VALUE_PROPERTY_WRITABLE as _;
    }
}

impl fmt::Display for ValuePropertyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ValuePropertyFlags {
    type GlibType = ffi::JSCValuePropertyFlags;

    fn into_glib(self) -> ffi::JSCValuePropertyFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::JSCValuePropertyFlags> for ValuePropertyFlags {
    unsafe fn from_glib(value: ffi::JSCValuePropertyFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

