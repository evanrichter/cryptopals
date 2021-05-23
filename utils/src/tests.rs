#![cfg(test)]

use super::*;

#[test]
fn set_1_challenge_1() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let mut bytes = Vec::new();
    hex_decode(hex, &mut bytes).unwrap();

    let encoded = base64_encode(&bytes);
    let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base64, encoded);
}

#[test]
fn base64() {
    let quote = "Man is distinguished, not only by his reason, but by this singular passion from other animals, \
             which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable \
             generation of knowledge, exceeds the short vehemence of any carnal pleasure.";

    let encoded = base64_encode(quote.as_bytes());

    let wikipedia = "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
                     IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
                     dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
                     dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
                     ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";

    assert_eq!(wikipedia, encoded, "wikipedia example");

    let mut decoded = Vec::new();
    let _ = base64_decode(wikipedia, &mut decoded).unwrap();

    assert_eq!(quote.as_bytes(), decoded, "wikipedia example");
}

#[test]
fn hex() {
    let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let mut decoded = Vec::new();
    let _ = base64_decode(base64, &mut decoded).unwrap();

    let hex_encoded = hex_encode(&decoded);
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!(hex, hex_encoded);
}
