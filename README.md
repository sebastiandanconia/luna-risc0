# Luna-RISC0: Trust-Minimized Security Background Investigation
## Synopsis
"Gilbert" is applying to be a partisan trying to overthrow the tyrannical Lunar Authority (science fiction, _The Moon is a Harsh Mistress_ by Robert A. Heinlein). He gathers various digital signatures on his Background Security Investigation document (I), and supplies (II) a `duress` boolean saying, in effect, "I'm compromised; don't trust me."
He then feeds these two sets of information into his local copy of the RISC Zero program supplied by the Party, which outputs a cryptographically verifiable decision on whether to admit him.

## Details
 The Party verifier knows based on the proof which of the following two conditions applies:
- All signatures on the Background Investigation are correct, and `duress` is false. (Alternative Hypothesis)
OR
- `duress` is true, or not all required signatures on the Background Investigation can be validated (Null Hypothesis)

Note that the verifier doesn't see the Background Investigation or the `duress` variable themselves.

### Public Inputs
- Adam Selene's public key (i.e. the Root of Trust)
- A representation of Gilbert's identity, such as a public key, or key fingerprint

### Private Inputs
- `duress` variable
- Most keys/signatures by witnesses/neighbors (Not yet implemented). Ultimately, we'd like to have the full certificate chain available for each signer of the Security Background Investigation. As a simplifying assumption, we currently require that each candidate is approved directly by Adam Selene, the computer who manages Party activities.

### Public Outputs
- `is_trustworthy`, representing whether "Gilbert" should be admitted to the Party
- Additionally, we echo the Public Inputs for the sake of verifiability. The RISC Zero API may have a better alternative to this approach.

## Building & Running
Checkout the source tree, and change to the source tree directory. You will need to install the Protocol Buffers compiler (`protoc`), and the Rust plugin for `protoc`. From `luna-risc0/sec_investigation/src/`, run `protoc --rust_out . sec_investigation.proto`. Then do `cargo run`. After several minutes, you should see something like the following:
```
=====================================================================================
= Luna shall be free! Please transmit proof of your Party Security Investigation.
=====================================================================================

******************************************************************************************
* Security investigation RESULT: is_trustworthy == true
*
* Public inputs:
* --------------
* Message: "Party name: 'Gilbert' is approved by Adam Selene for party membership."
* Key: 033F690F9CF7D18C43085DF78916E4E37F459E1350B339CA3DBE93944AE75C201F
******************************************************************************************
```

## Possible Enhancements
A more complete Security Background Investigation would include some of the following:
- Support a chain of trust signatures. For example, Adam can sign Dylan's public key, and Dylan can sign Gilbert's public key.
- Establish history of the person's identity. Any time they interacted with credible witnesses/neighbors, that's an event that could be used to validate that they didn't invent their persona. Photographic + name comparisons are preferable to name only.
- AI/ML analysis of biometric inputs (photos, fingerprints, etc.)
- Graph analysis of the trustworthiness of the people attesting to the applicant's loyalties, which could use Personalized PageRank, or other forms of recursion.
- Multiple signatures, each one representing a witness (for example, who attests that the applicant lived at the address(es) he/she claims)
- Recursive proofs/signatures could be used, particularly when incorporating outputs from ML systems.
