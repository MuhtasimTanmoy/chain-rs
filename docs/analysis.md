
`cargo run printchain`

will provide the following log.
There are two blocks.
- One, coinbase transaction that gives the creator of the chain 100 tokens
- Two, when a user sends 10 token to another user and signs that

## TODO
- Describe the flow of signature, pub_key, pub_key_hash

```
block: Block {
    timestamp: 1677343767216,
    hash: "0000d1ab72096f70b2f6510d3964948a79c2937da3f30d334f54ae23ff379fd7",
    hash_prev_block: "000046b486d72c8d1d95df54ecd5aad8c82976b7f62c98589f405e783d17a0f3",
    transactions: [
        Transaction {
            id: "93418da2ce6e109379f8773d5e30b6cd2a439811b1ac666e86a4bae29b66df1d",
            input: [
                TXInput {
                    txid: "f6f4c381de0fc1e15fa17409b5a4f81272ccac752a0924daf3c9e15bac5f554e",
                    vout: 0,
                    signature: [
                        138,
                        129,
                        183,
                        153,
                        149,
                        99,
                        99,
                        130,
                        27,
                        52,
                        153,
                        92,
                        91,
                        19,
                        249,
                        68,
                        27,
                        18,
                        1,
                        244,
                        169,
                        172,
                        167,
                        154,
                        69,
                        223,
                        207,
                        42,
                        58,
                        18,
                        197,
                        139,
                        23,
                        240,
                        229,
                        120,
                        5,
                        152,
                        78,
                        215,
                        213,
                        249,
                        211,
                        244,
                        155,
                        69,
                        13,
                        249,
                        205,
                        56,
                        185,
                        110,
                        112,
                        226,
                        187,
                        73,
                        212,
                        220,
                        89,
                        206,
                        43,
                        191,
                        204,
                        9,
                    ],
                    pub_key: [
                        130,
                        134,
                        54,
                        193,
                        33,
                        5,
                        57,
                        179,
                        98,
                        16,
                        39,
                        176,
                        206,
                        140,
                        185,
                        197,
                        191,
                        134,
                        17,
                        85,
                        121,
                        215,
                        209,
                        201,
                        106,
                        139,
                        199,
                        93,
                        43,
                        242,
                        96,
                        62,
                    ],
                },
            ],
            output: [
                TXOutput {
                    value: 10,
                    pub_key_hash: [
                        51,
                        20,
                        67,
                        147,
                        223,
                        175,
                        193,
                        183,
                        17,
                        193,
                        228,
                        117,
                        163,
                        148,
                        102,
                        5,
                        153,
                        153,
                        72,
                        69,
                    ],
                },
                TXOutput {
                    value: 90,
                    pub_key_hash: [
                        181,
                        201,
                        77,
                        9,
                        214,
                        30,
                        77,
                        175,
                        225,
                        138,
                        198,
                        105,
                        85,
                        174,
                        6,
                        72,
                        186,
                        158,
                        96,
                        181,
                    ],
                },
            ],
        },
    ],
    nonce: 63089,
    height: 0,
    version: 1,
    difficulty: 4,
}
block: Block {
    timestamp: 1677343656943,
    hash: "000046b486d72c8d1d95df54ecd5aad8c82976b7f62c98589f405e783d17a0f3",
    hash_prev_block: "",
    transactions: [
        Transaction {
            id: "f6f4c381de0fc1e15fa17409b5a4f81272ccac752a0924daf3c9e15bac5f554e",
            input: [
                TXInput {
                    txid: "",
                    vout: -1,
                    signature: [],
                    pub_key: [
                        83,
                        111,
                        109,
                        101,
                        32,
                        100,
                        97,
                        116,
                        97,
                        32,
                        102,
                        111,
                        114,
                        32,
                        103,
                        101,
                        110,
                        101,
                        115,
                        105,
                        115,
                        32,
                        98,
                        108,
                        111,
                        99,
                        107,
                    ],
                },
            ],
            output: [
                TXOutput {
                    value: 100,
                    pub_key_hash: [
                        181,
                        201,
                        77,
                        9,
                        214,
                        30,
                        77,
                        175,
                        225,
                        138,
                        198,
                        105,
                        85,
                        174,
                        6,
                        72,
                        186,
                        158,
                        96,
                        181,
                    ],
                },
            ],
        },
    ],
    nonce: 15156,
    height: 0,
    version: 1,
    difficulty: 4,
}

```