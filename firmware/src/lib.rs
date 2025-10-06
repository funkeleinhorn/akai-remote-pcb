#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![feature(future_join)]

// SPDX-FileCopyrightText: 2025 Funkeleinhorn <git@funkeleinhorn.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod gpio;
pub mod web;
pub mod wifi;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
