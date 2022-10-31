/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2022 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::constants::{
    fixed_bases::{
        VALUE_COMMITMENT_PERSONALIZATION, VALUE_COMMITMENT_R_BYTES, VALUE_COMMITMENT_V_BYTES,
    },
    NullifierK,
};
use halo2_gadgets::ecc::chip::FixedPoint;
use pasta_curves::{arithmetic::CurveExt, group::ff::PrimeField, pallas};

pub type ValueBlind = pallas::Scalar;
pub type ValueCommit = pallas::Point;

/// Pedersen commitment for a full-width base field element.
#[allow(non_snake_case)]
pub fn pedersen_commitment_base(value: pallas::Base, blind: ValueBlind) -> ValueCommit {
    let hasher = ValueCommit::hash_to_curve(VALUE_COMMITMENT_PERSONALIZATION);
    let V = NullifierK.generator();
    let R = hasher(&VALUE_COMMITMENT_R_BYTES);

    V * mod_r_p(value) + R * blind
}

/// Pedersen commitment for a 64-bit value, in the base field.
#[allow(non_snake_case)]
pub fn pedersen_commitment_u64(value: u64, blind: ValueBlind) -> ValueCommit {
    let hasher = ValueCommit::hash_to_curve(VALUE_COMMITMENT_PERSONALIZATION);
    let V = hasher(&VALUE_COMMITMENT_V_BYTES);
    let R = hasher(&VALUE_COMMITMENT_R_BYTES);

    V * mod_r_p(pallas::Base::from(value)) + R * blind
}

/// Converts from pallas::Base to pallas::Scalar (aka $x \pmod{r_\mathbb{P}}$).
///
/// This requires no modular reduction because Pallas' base field is smaller than its
/// scalar field.
pub fn mod_r_p(x: pallas::Base) -> pallas::Scalar {
    pallas::Scalar::from_repr(x.to_repr()).unwrap()
}
