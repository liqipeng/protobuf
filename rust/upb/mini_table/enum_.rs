// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#[repr(C)]
pub struct MiniTableEnum {
    mask_limit: u32,
    value_count: u32,
    // TODO: We need to figure out how to model the flexible array member in Rust.
    // data: [u32],
}
