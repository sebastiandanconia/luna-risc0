use methods::{LUNA_ELF, LUNA_ID};

use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    EncodedPoint,
};
use rand_core::OsRng;
use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv, Receipt,
};

use sec_investigation::*;

fn zkvm_template_main() {
    // First, we construct an executor environment
    let env = ExecutorEnv::builder().build().unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, LUNA_ELF).unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(LUNA_ID).unwrap();
}

fn main() {
    println!("\n=====================================================================================");
    println!("= Luna shall be free! Please transmit proof of your Party Security Investigation.");
    println!("=====================================================================================\n");
    ecdsa_host_demo();
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


fn prove_security_investigation(
    duress: bool,
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> Receipt {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&duress).unwrap())
        .add_input(&to_vec(&(verifying_key.to_encoded_point(true), message, signature)).unwrap())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    prover.prove_elf(env, LUNA_ELF).unwrap()
}

fn ecdsa_host_demo() {
    // Generate a random secp256k1 keypair and sign the message.
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"Party name: 'Gilbert' is approved by Adam Selene for party membership.";
    let signature: Signature = signing_key.sign(message);

    // Applicant: Set to true if you are compromised or under duress by Lunar Authority
    let duress = false;

    // Run signature verified in the zkVM guest and get the resulting receipt.
    let receipt = prove_security_investigation(duress, signing_key.verifying_key(), message, &signature);

    // Verify the receipt and then access the journal.
    receipt.verify(LUNA_ID).unwrap();
    let (is_trustworthy, receipt_verifying_key, receipt_message) =
        from_slice::<(bool, EncodedPoint, Vec<u8>), _>(&receipt.journal)
            .unwrap()
            .try_into()
            .unwrap();

    println!("******************************************************************************************");
    println!("* Security investigation RESULT: is_trustworthy == {}", is_trustworthy);
    println!("*");
    println!("* Public inputs:");
    println!("* --------------");
    println!("* Message: {:?}\n* Key: {}",
        std::str::from_utf8(&receipt_message[..]).unwrap(),
        receipt_verifying_key,
    );
    println!("******************************************************************************************");
}
////////////////////////////////////////////////////////////////////////////////
