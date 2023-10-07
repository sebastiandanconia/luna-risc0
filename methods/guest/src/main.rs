#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std]  // std support is experimental

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};
use risc0_zkvm::guest::env;

use sec_investigation::*;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    ecdsa_demo();
}


////////////////////////////////////////////////////////////////////////////////
// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


fn ecdsa_demo() {
    // Decode the verifying key, message, and signature from the inputs.
    let (duress, encoded_verifying_key, message, signature): (bool, EncodedPoint, Vec<u8>, Signature) =
        env::read();
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    let good_signature = verifying_key
        .verify(&message, &signature).is_ok();

    // Do signature verification, and also support duress functionality
    let is_trustworthy = good_signature && !duress;

    // Commit to the journal the result of the background investigation.
    // Echoing the public inputs is a form of protection against tampering.
    env::commit(&(is_trustworthy, encoded_verifying_key, message));
}
////////////////////////////////////////////////////////////////////////////////
