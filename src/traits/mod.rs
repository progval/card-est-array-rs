/*
 * SPDX-FileCopyrightText: 2024 Matteo Dell'Acqua
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

mod estimator;
pub use estimator::*;
mod estimator_array;
pub use estimator_array::*;

use num_primitive::PrimitiveUnsigned;
use num_traits::{ConstOne, ConstZero};

/// A convenience trait bundling the bounds required for word types used as
/// backends for estimators.
pub trait Word: PrimitiveUnsigned + ConstZero + ConstOne {}
impl<W: PrimitiveUnsigned + ConstZero + ConstOne> Word for W {}
