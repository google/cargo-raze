// Copyright (c) 2016 George Burton
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION

// Copyright (c) 1985-2008 by Apple Inc.. All rights reserved.
// @APPLE_LICENSE_HEADER_START@
//
// This file contains Original Code and/or Modifications of Original Code
// as defined in and that are subject to the Apple Public Source License
// Version 2.0 (the 'License'). You may not use this file except in
// compliance with the License. Please obtain a copy of the License at
// http://www.opensource.apple.com/apsl/ and read it before using this
// file.
//
// The Original Code and all software distributed under the License are
// distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
// INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
// Please see the License for the specific language governing rights and
// limitations under the License.
//
// @APPLE_LICENSE_HEADER_END@

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![cfg_attr(not(feature = "use_std"), no_std)]

//! The `MacTypes-sys` library provides bindings to the `MacTypes.h` header on MacOS.
//! This library defines base types used in both Carbon and legacy Cocoa APIs.

extern crate libc;

#[cfg(feature = "use_std")]
extern crate core;

use core::cmp::{Eq, PartialEq};
use core::hash::{Hash, Hasher};
use core::{fmt, mem, ptr, str};
use libc::*;

/// 8-bit unsigned integer.
pub type UInt8 = u8;
/// 8-bit signed integer.
pub type SInt8 = i8;
/// 16-bit unsigned integer.
pub type UInt16 = u16;
/// 16-bit signed integer.
pub type SInt16 = i16;
/// 32-bit unsigned integer.
pub type UInt32 = u32;
/// 32-bit signed integer.
pub type SInt32 = i32;

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct wide {
    pub hi: SInt32,
    pub lo: UInt32,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnsignedWide {
    pub hi: UInt32,
    pub lo: UInt32,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct wide {
    pub lo: UInt32,
    pub hi: SInt32,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnsignedWide {
    pub lo: UInt32,
    pub hi: UInt32,
}

/// 64-bit signed integer.
pub type SInt64 = i64;
/// 64-bit unsigned integer.
pub type UInt64 = u64;

/// 16-bit signed integer plus 16-bit fraction.
pub type Fixed = SInt32;
/// A pointer to a `Fixed`.
pub type FixedPtr = *mut Fixed;
/// 2-bit signed integer plus 30-bit fraction.
pub type Fract = SInt32;
/// A pointer to a `Fract`.
pub type FractPtr = *mut Fract;
/// 16-bit unsigned integer plus 16-bit fraction.
pub type UnsignedFixed = UInt32;
/// A pointer to an `UnsignedFixed`.
pub type UnsignedFixedPtr = *mut UnsignedFixed;
/// 8-bit signed integer plus 8-bit fraction.
pub type ShortFixed = SInt16;
/// A pointer to a `ShortFixed`.
pub type ShortFixedPtr = *mut ShortFixed;

/// 32 bit IEEE float:  1 sign bit, 8 exponent bits, 23 fraction bits.
pub type Float32 = f32;
/// 64 bit IEEE float:  1 sign bit, 11 exponent bits, 52 fraction bits.
pub type Float64 = f64;

/// 80 bit MacOS float: 1 sign bit, 15 exponent bits, 1 integer bit, 63 fraction bits.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Float80 {
    pub exp: SInt16,
    pub man: [UInt16; 4],
}

/// 96 bit 68881 float: 1 sign bit, 15 exponent bits, 16 pad bits, 1 integer bit, 63
/// fraction bits.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Float96 {
    pub exp: [SInt16; 2],
    pub man: [UInt16; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Float32Point {
    pub x: Float32,
    pub y: Float32,
}

/// Pointer to a non-relocatable block.
pub type Ptr = *mut c_char;
/// Pointer to a master pointer to a relocatable block.
pub type Handle = *mut Ptr;
/// The number of bytes in a block (signed for historical reasons).
pub type size = c_long;

/// 16-bit result error code.
pub type OSErr = SInt16;
/// 32-bit result error code.
pub type OSStatus = SInt32;
/// Address in the clients virtual address space.
pub type LogicalAddress = *mut c_void;
/// Address in the clients virtual address space that will only be read.
pub type ConstLogicalAddress = *const c_void;
/// Real address as used on the hardware bus.
pub type PhysicalAddress = *mut c_void;
/// Pointer to an array of bytes.
pub type BytePtr = *mut UInt8;
/// The size of an array of bytes.
pub type ByteCount = c_ulong;
/// An offset into an array of bytes.
pub type ByteOffset = c_ulong;
/// 32-bit millisecond timer for drivers.
pub type Duration = SInt32;
/// 64-bit clock.
pub type AbsoluteTime = UnsignedWide;
/// Standard 32-bit set of bit flags.
pub type OptionBits = UInt32;
/// 32-bit iteration count.
pub type ItemCount = c_ulong;
/// ?
pub type PBVersion = UInt32;
/// A particular set of written characters (e.g. Roman vs Cyrillic) and their encoding.
pub type ScriptCode = SInt16;
/// A particular language (e.g. English), as represented using a particular ScriptCode.
pub type LangCode = SInt16;
/// Designates a language as used in a particular region (e.g. British vs American
/// English) together with other region-dependent characteristics (e.g. date format).
pub type RegionCode = SInt16;
/// A 32-bit value made by packing four 1 byte characters together.
pub type FourCharCode = UInt32;
/// A `FourCharCode` used in the OS and file system (e.g. creator).
pub type OSType = FourCharCode;
/// A `FourCharCode` used to tag resources (e.g. `DLOG`).
pub type ResType = FourCharCode;
/// A pointer to an `OSType`.
pub type OSTypePtr = *mut OSType;
/// A pointer to a `ResType`.
pub type ResTypePtr = *mut ResType;

/// Mac OS historic type, `sizeof(Boolean)==1`.
pub type Boolean = c_uchar;

/// Generic pointer to a function.
pub type ProcPtr = unsafe extern "C" fn(c_long);
/// Pointer to a 68K function that expects parameters in registers.
pub type Register68kProcPtr = unsafe extern "C" fn();

/// Pointer to classic 68K code or a `RoutineDescriptor`.
pub type UniversalProcPtr = ProcPtr;

/// Pointer to a `ProcPtr`.
pub type ProcHandle = *mut ProcPtr;
/// Pointer to a `UniversalProcPtr`.
pub type UniversalProcHandle = *mut UniversalProcPtr;

/// # RefCon Types
///
/// For access to private data in callbacks, etc.; refcons are generally
/// used as a pointer to something, but in the 32-bit world refcons in
/// different APIs have had various types: pointer, unsigned scalar, and
/// signed scalar. The RefCon types defined here support the current 32-bit
/// usage but provide normalization to pointer types for 64-bit.
///
/// `PRefCon` is preferred for new APIs; `URefCon` and `SRefCon` are primarily
/// for compatibility with existing APIs.
pub type PRefCon = *mut c_void;

/// # RefCon Types
///
/// For access to private data in callbacks, etc.; refcons are generally
/// used as a pointer to something, but in the 32-bit world refcons in
/// different APIs have had various types: pointer, unsigned scalar, and
/// signed scalar. The RefCon types defined here support the current 32-bit
/// usage but provide normalization to pointer types for 64-bit.
///
/// `PRefCon` is preferred for new APIs; `URefCon` and `SRefCon` are primarily
/// for compatibility with existing APIs.
#[cfg(target_pointer_width = "64")]
pub type URefCon = *mut c_void;

/// # RefCon Types
///
/// For access to private data in callbacks, etc.; refcons are generally
/// used as a pointer to something, but in the 32-bit world refcons in
/// different APIs have had various types: pointer, unsigned scalar, and
/// signed scalar. The RefCon types defined here support the current 32-bit
/// usage but provide normalization to pointer types for 64-bit.
///
/// `PRefCon` is preferred for new APIs; `URefCon` and `SRefCon` are primarily
/// for compatibility with existing APIs.
#[cfg(target_pointer_width = "64")]
pub type SRefCon = *mut c_void;

/// # RefCon Types
///
/// For access to private data in callbacks, etc.; refcons are generally
/// used as a pointer to something, but in the 32-bit world refcons in
/// different APIs have had various types: pointer, unsigned scalar, and
/// signed scalar. The RefCon types defined here support the current 32-bit
/// usage but provide normalization to pointer types for 64-bit.
///
/// `PRefCon` is preferred for new APIs; `URefCon` and `SRefCon` are primarily
/// for compatibility with existing APIs.
#[cfg(target_pointer_width = "32")]
pub type URefCon = UInt32;

/// # RefCon Types
///
/// For access to private data in callbacks, etc.; refcons are generally
/// used as a pointer to something, but in the 32-bit world refcons in
/// different APIs have had various types: pointer, unsigned scalar, and
/// signed scalar. The RefCon types defined here support the current 32-bit
/// usage but provide normalization to pointer types for 64-bit.
///
/// `PRefCon` is preferred for new APIs; `URefCon` and `SRefCon` are primarily
/// for compatibility with existing APIs.
#[cfg(target_pointer_width = "32")]
pub type SRefCon = SInt32;

/// `OSErr`: function performed properly - no error.
pub const kNoErr: OSErr = 0;
/// `OptionBits`: all flags false
pub const kNilOptions: OptionBits = 0;
/// `KernelID`: `NULL` is for pointers as `kInvalidID` is for ID's
pub const kInvalidId: u32 = 0;
/// Array bounds: variable length array
///
/// # Note
///
/// `kVariableLengthArray` is used in array bounds to specify a variable length array.
/// It is ususally used in variable length structs when the last field is an array
/// of any size.  Before ANSI C, we used zero as the bounds of variable length
/// array, but zero length array are illegal in ANSI C.
///
/// ## Example usage:
///
/// ```c
/// struct FooList
/// {
///     short   listLength;
///     Foo     elements[kVariableLengthArray];
/// };
/// ```
pub const kVariableLengthArray: ItemCount = 1;
/// "????" QuickTime 3.0: default unknown `ResType` or `OSType`
pub const kUnknownType: OSType = 0x3F3F3F3F;

/// A complete Unicode character in UTF-32 format, with
/// values from `0` through `0x10FFFF` (excluding the surrogate
/// range 0xD800-0xDFFF and certain disallowed values).
pub type UnicodeScalarValue = UInt32;
/// A complete Unicode character in UTF-32 format, with
/// values from `0` through `0x10FFFF` (excluding the surrogate
/// range `0xD800`-`0xDFFF` and certain disallowed values).
pub type UTF32Char = UInt32;
/// A 16-bit Unicode code value in the default UTF-16 format.
/// UnicodeScalarValues `0`-`0xFFFF` are expressed in UTF-16
/// format using a single `UTF16Char` with the same value.
/// UnicodeScalarValues `0x10000`-`0x10FFFF` are expressed in
/// UTF-16 format using a pair of `UTF16Char`s - one in the
/// high surrogate range (`0xD800`-`0xDBFF`) followed by one in
/// the low surrogate range (`0xDC00`-`0xDFFF`). All of the
/// characters defined in Unicode versions through 3.0 are
/// in the range `0`-`0xFFFF` and can be expressed using a single
/// `UTF16Char`, thus the term "Unicode character" generally
/// refers to a `UniChar` = `UTF16Char`.
pub type UniChar = UInt16;
/// A 16-bit Unicode code value in the default UTF-16 format.
/// UnicodeScalarValues `0`-`0xFFFF` are expressed in UTF-16
/// format using a single `UTF16Char` with the same value.
/// UnicodeScalarValues `0x10000`-`0x10FFFF` are expressed in
/// UTF-16 format using a pair of `UTF16Char`s - one in the
/// high surrogate range (`0xD800`-`0xDBFF`) followed by one in
/// the low surrogate range (`0xDC00`-`0xDFFF`). All of the
/// characters defined in Unicode versions through 3.0 are
/// in the range `0`-`0xFFFF` and can be expressed using a single
/// `UTF16Char`, thus the term "Unicode character" generally
/// refers to a `UniChar` = `UTF16Char`.
pub type UTF16Char = UInt16;
/// An 8-bit code value in UTF-8 format. `UnicodeScalarValue`s
/// `0`-`0x7F` are expressed in UTF-8 format using one `UTF8Char`
/// with the same value. `UnicodeScalarValue`s above `0x7F` are
/// expressed in UTF-8 format using 2-4 `UTF8Char`s, all with
/// values in the range `0x80`-`0xF4` (`UnicodeScalarValue`s
/// `0x100`-`0xFFFF` use two or three `UTF8Char`s,
/// `UnicodeScalarValue`s `0x10000`-`0x10FFFF` use four `UTF8Chars`).
pub type UTF8Char = UInt8;
/// A pointer to an array of `UniChar`s.
pub type UniCharPtr = *mut UniChar;
/// A count of UTF-16 code values in an array or buffer.
pub type UniCharCount = c_ulong;
/// A pointer to a `UniCharCount`.
pub type UniCharCountPtr = *mut UniCharCount;
/// Pascal string holding up to 255 bytes
pub type Str255 = [c_uchar; 256];
/// Pascal string holding up to 63 bytes
pub type Str63 = [c_uchar; 64];
/// Pascal string holding up to 31 bytes
pub type Str31 = [c_uchar; 32];
/// Pascal string holding up to 27 bytes
pub type Str27 = [c_uchar; 28];
/// Pascal string holding up to 15 bytes
pub type Str15 = [c_uchar; 16];

/// The type `Str32` is used in many AppleTalk based data structures.
/// It holds up to 32 one byte chars.  The problem is that with the
/// length byte it is 33 bytes long.  This can cause weird alignment
/// problems in structures.  To fix this the type `Str32Field` has
/// been created.  It should only be used to hold 32 chars, but
/// it is 34 bytes long so that there are no alignment problems.
pub type Str32Field = [c_uchar; 34];

/// QuickTime 3.0:
///
/// The type `StrFileName` is used to make MacOS structs work
/// cross-platform.  For example `FSSpec` or `SFReply` previously
/// contained a `Str63` field.  They now contain a `StrFileName`
/// field which is the same when targeting the MacOS but is
/// a 256 char buffer for Win32 and unix, allowing them to
/// contain long file names.
pub type StrFileName = Str63;

/// Pointer to a pascal string.
pub type StringPtr = *mut c_uchar;
/// Pointer to a `StringPtr`.
pub type StringHandle = *mut StringPtr;
/// Pointer to a read-only pascal string.
pub type ConstStringPtr = *const c_uchar;
/// For function parameters only - means string is const.
pub type ConstStr255Param = *const c_uchar;
/// For function parameters only - means string is const.
pub type ConstStr63Param = *const c_uchar;
/// For function parameters only - means string is const.
pub type ConstStr31Param = *const c_uchar;
/// For function parameters only - means string is const.
pub type ConstStr27Param = *const c_uchar;
/// For function parameters only - means string is const.
pub type ConstStr15Param = *const c_uchar;
/// For function parameters only - means string is const.
pub type ConstStrFileNameParam = *const ConstStr63Param;

/// Get the length of a pascal string.
#[inline(always)]
pub unsafe fn StrLength(string: ConstStr255Param) -> c_uchar {
    *string
}

/// Type for unique process identifier.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ProcessSerialNumber {
    pub highLongOfPSN: UInt32,
    pub lowLongOfPSN: UInt32,
}

pub type ProcessSerialNumberPtr = *mut ProcessSerialNumber;

/// 2D Quickdraw coordinate, range: -32K to +32K.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point {
    pub v: c_short,
    pub h: c_short,
}

pub type PointPtr = *mut Point;

/// Rectangular Quickdraw area.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Rect {
    pub top: c_short,
    pub left: c_short,
    pub bottom: c_short,
    pub right: c_short,
}

pub type RectPtr = *mut Rect;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FixedRect {
    pub top: Fixed,
    pub left: Fixed,
    pub bottom: Fixed,
    pub right: Fixed,
}

pub type FixedRectPtr = *mut FixedRect;

/// `Char` when used as a parameter (historical 68K convention).
pub type CharParameter = c_short;

pub const normal: Style = 0;
pub const bold: Style = 1;
pub const italic: Style = 2;
pub const underline: Style = 4;
pub const outline: Style = 8;
pub const shadow: Style = 0x10;
pub const condense: Style = 0x20;
pub const extend: Style = 0x40;

/// Quickdraw font rendering styles.
///
/// # Note
///
/// The original Macintosh toolbox in 68K Pascal defined `Style` as a SET.
/// Both `Style` and `CHAR` occupy 8-bits in packed records or 16-bits when
/// used as fields in non-packed records or as parameters.
pub type Style = c_uchar;
/// `Style` when used as a parameter (historical 68K convention).
pub type StyleParameter = c_short;
/// `Style` when used as a field (historical 68K convention).
pub type StyleField = Style;

/// Count of units.
pub type TimeValue = SInt32;
/// Units per second.
pub type TimeScale = SInt32;
/// 64-bit count of units (always a struct).
pub type CompTimeValue = wide;
/// 64-bit count of units (long long or struct).
pub type TimeValue64 = SInt64;
/// An opaque reference to a time base.
pub type TimeBase = *mut TimeBaseRecord;

#[doc(hidden)]
#[repr(C)]
pub struct TimeBaseRecord {
    _priv: c_void,
}

/// Package of TimeBase, duration, and scale.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TimeRecord {
    /// Units (duration or absolute).
    pub value: CompTimeValue,
    /// Units per second.
    pub scale: TimeScale,
    /// Reference to the time base
    pub base: TimeBase,
}

/// Packed BCD version representation (e.g. "4.2.1a3" is `0x04214003`).
#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NumVersion {
    /// 1st part of version number in BCD.
    pub majorRev: UInt8,
    /// 2nd & 3rd part of version number share a byte.
    pub minorAndBugRef: UInt8,
    /// stage code: dev, alpha, beta, final.
    pub stage: UInt8,
    /// revision level of non-released version.
    pub nonRelRev: UInt8,
}

/// Packed BCD version representation (e.g. "4.2.1a3" is `0x04214003`).
#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NumVersion {
    /// revision level of non-released version.
    pub nonRelRev: UInt8,
    /// stage code: dev, alpha, beta, final.
    pub stage: UInt8,
    /// 2nd & 3rd part of version number share a byte.
    pub minorAndBugRef: UInt8,
    /// 1st part of version number in BCD.
    pub majorRev: UInt8,
}

/// Version Release Stage Code.
pub const developStage: UInt8 = 0x20;
/// Version Release Stage Code.
pub const alphaStage: UInt8 = 0x40;
/// Version Release Stage Code.
pub const betaStage: UInt8 = 0x60;
/// Version Release Stage Code.
pub const finalStage: UInt8 = 0x80;

/// `NumVersionVariant` is a wrapper so `NumVersion` can be accessed as a 32-bit value.
#[repr(C)]
#[derive(Clone, Copy)]
pub union NumVersionVariant {
    pub parts: NumVersion,
    pub whole: UInt32,
}

impl Default for NumVersionVariant {
    #[inline]
    fn default() -> Self {
        NumVersionVariant { whole: 0 }
    }
}

impl fmt::Debug for NumVersionVariant {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let parts = unsafe { self.parts };
        fmtr.debug_struct("NumVersionVariant")
            .field("parts", &parts)
            .finish()
    }
}

impl PartialEq for NumVersionVariant {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.whole.eq(&other.whole) }
    }
}

impl PartialEq<NumVersion> for NumVersionVariant {
    #[inline]
    fn eq(&self, other: &NumVersion) -> bool {
        unsafe { self.parts.eq(&other) }
    }
}

impl PartialEq<UInt32> for NumVersionVariant {
    #[inline]
    fn eq(&self, other: &UInt32) -> bool {
        unsafe { self.whole.eq(&other) }
    }
}

impl Eq for NumVersionVariant {}

impl Hash for NumVersionVariant {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let whole = unsafe { self.whole };
        state.write_u32(whole);
    }
}

pub type NumVersionVariantPtr = *mut NumVersionVariant;
pub type NumVersionVariantHandle = *mut NumVersionVariantPtr;

/// Contents of a `vers` resource.
#[repr(C)]
pub struct VersRec {
    pub numericVersion: NumVersion,
    pub countryCode: c_short,
    pub shortVersion: Str255,
    pub reserved: Str255,
}

// Manual implementation of various traits on `VersRec` as the `Str255` members prevent auto-derive

impl Clone for VersRec {
    fn clone(&self) -> Self {
        let mut clone = VersRec::default();
        clone.numericVersion = self.numericVersion;
        clone.countryCode = self.countryCode;
        unsafe {
            ptr::copy_nonoverlapping(&self.shortVersion, &mut clone.shortVersion, 1);
            ptr::copy_nonoverlapping(&self.reserved, &mut clone.reserved, 1);
        }
        clone
    }
}

impl Copy for VersRec {}

impl Default for VersRec {
    #[inline]
    fn default() -> Self {
        VersRec {
            numericVersion: Default::default(),
            countryCode: Default::default(),
            shortVersion: [0; 256],
            reserved: [0; 256],
        }
    }
}

impl Eq for VersRec {}

impl fmt::Debug for VersRec {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("VersRec")
            .field("numericVersion", &self.numericVersion)
            .field("countryCode", &self.countryCode)
            .field(
                "shortVersion",
                &str::from_utf8(&self.shortVersion).map_err(|_| fmt::Error)?,
            )
            .field(
                "reserved",
                &str::from_utf8(&self.reserved).map_err(|_| fmt::Error)?,
            )
            .finish()
    }
}

impl Hash for VersRec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numericVersion.hash(state);
        state.write_i16(self.countryCode);
        state.write(&self.shortVersion);
        state.write(&self.reserved);
    }
}

impl PartialEq for VersRec {
    fn eq(&self, other: &Self) -> bool {
        #[inline]
        fn ptr<T>(slice: &[T]) -> *const c_void {
            slice.as_ptr() as *const _
        }

        unsafe {
            self.numericVersion == other.numericVersion && self.countryCode == other.countryCode &&
                memcmp(
                    ptr(&self.shortVersion),
                    ptr(&other.shortVersion),
                    mem::size_of::<Str255>(),
                ) == 0 &&
                memcmp(
                    ptr(&self.reserved),
                    ptr(&other.reserved),
                    mem::size_of::<Str255>(),
                ) == 0
        }
    }
}

/// Pointer to a `VersRecPtr`.
pub type VersRecPtr = *mut VersRec;
/// Resource Handle containing a `VersRec`.
pub type VersRecHndl = *mut VersRecPtr;

pub type Byte = UInt8;
pub type SignedByte = SInt8;
pub type WidePtr = *mut wide;
pub type UnsignedWidePtr = *mut UnsignedWide;
pub type extended80 = Float80;
pub type extended96 = Float96;
pub type VHSelect = SInt8;

#[cfg(target_os = "macos")]
#[cfg_attr(target_os = "macos", link(name = "CoreServices", kind = "framework"))]
extern "C" {
    pub fn Debugger();
    pub fn DebugStr(debuggerMsg: ConstStr255Param);
    pub fn SysBreak();
    pub fn SysBreakStr(debuggerMsg: ConstStr255Param);
    pub fn SysBreakFunc(debuggerMsg: ConstStr255Param);
}

#[cfg(test)]
mod versrec_manual_derive_tests {
    use super::*;
    #[test]
    fn test_clone_and_eq() {
        let mut rec0 = VersRec::default();
        rec0.countryCode = 14;

        let random_string = b"Hello, world!A)R\xE2Q$";
        for i in 0..random_string.len() {
            rec0.shortVersion[i] = random_string[i];
        }

        let rec1 = rec0.clone();
        assert_eq!(rec0, rec1);
    }
}
