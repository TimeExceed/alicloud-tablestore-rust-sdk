use crate::plainbuffer as pbuf;

#[test]
fn crc8_u8() {
    const ORACLES: [u8; 256] = [
        0x0u8, 0x7u8, 0xEu8, 0x9u8, 0x1Cu8, 0x1Bu8, 0x12u8, 0x15u8,
        0x38u8, 0x3Fu8, 0x36u8, 0x31u8, 0x24u8, 0x23u8, 0x2Au8, 0x2Du8,
        0x70u8, 0x77u8, 0x7Eu8, 0x79u8, 0x6Cu8, 0x6Bu8, 0x62u8, 0x65u8,
        0x48u8, 0x4Fu8, 0x46u8, 0x41u8, 0x54u8, 0x53u8, 0x5Au8, 0x5Du8,

        0xE0u8, 0xE7u8, 0xEEu8, 0xE9u8, 0xFCu8, 0xFBu8, 0xF2u8, 0xF5u8,
        0xD8u8, 0xDFu8, 0xD6u8, 0xD1u8, 0xC4u8, 0xC3u8, 0xCAu8, 0xCDu8,
        0x90u8, 0x97u8, 0x9Eu8, 0x99u8, 0x8Cu8, 0x8Bu8, 0x82u8, 0x85u8,
        0xA8u8, 0xAFu8, 0xA6u8, 0xA1u8, 0xB4u8, 0xB3u8, 0xBAu8, 0xBDu8,

        0xC7u8, 0xC0u8, 0xC9u8, 0xCEu8, 0xDBu8, 0xDCu8, 0xD5u8, 0xD2u8,
        0xFFu8, 0xF8u8, 0xF1u8, 0xF6u8, 0xE3u8, 0xE4u8, 0xEDu8, 0xEAu8,
        0xB7u8, 0xB0u8, 0xB9u8, 0xBEu8, 0xABu8, 0xACu8, 0xA5u8, 0xA2u8,
        0x8Fu8, 0x88u8, 0x81u8, 0x86u8, 0x93u8, 0x94u8, 0x9Du8, 0x9Au8,

        0x27u8, 0x20u8, 0x29u8, 0x2Eu8, 0x3Bu8, 0x3Cu8, 0x35u8, 0x32u8,
        0x1Fu8, 0x18u8, 0x11u8, 0x16u8, 0x3u8, 0x4u8, 0xDu8, 0xAu8,
        0x57u8, 0x50u8, 0x59u8, 0x5Eu8, 0x4Bu8, 0x4Cu8, 0x45u8, 0x42u8,
        0x6Fu8, 0x68u8, 0x61u8, 0x66u8, 0x73u8, 0x74u8, 0x7Du8, 0x7Au8,

        0x89u8, 0x8Eu8, 0x87u8, 0x80u8, 0x95u8, 0x92u8, 0x9Bu8, 0x9Cu8,
        0xB1u8, 0xB6u8, 0xBFu8, 0xB8u8, 0xADu8, 0xAAu8, 0xA3u8, 0xA4u8,
        0xF9u8, 0xFEu8, 0xF7u8, 0xF0u8, 0xE5u8, 0xE2u8, 0xEBu8, 0xECu8,
        0xC1u8, 0xC6u8, 0xCFu8, 0xC8u8, 0xDDu8, 0xDAu8, 0xD3u8, 0xD4u8,

        0x69u8, 0x6Eu8, 0x67u8, 0x60u8, 0x75u8, 0x72u8, 0x7Bu8, 0x7Cu8,
        0x51u8, 0x56u8, 0x5Fu8, 0x58u8, 0x4Du8, 0x4Au8, 0x43u8, 0x44u8,
        0x19u8, 0x1Eu8, 0x17u8, 0x10u8, 0x5u8, 0x2u8, 0xBu8, 0xCu8,
        0x21u8, 0x26u8, 0x2Fu8, 0x28u8, 0x3Du8, 0x3Au8, 0x33u8, 0x34u8,

        0x4Eu8, 0x49u8, 0x40u8, 0x47u8, 0x52u8, 0x55u8, 0x5Cu8, 0x5Bu8,
        0x76u8, 0x71u8, 0x78u8, 0x7Fu8, 0x6Au8, 0x6Du8, 0x64u8, 0x63u8,
        0x3Eu8, 0x39u8, 0x30u8, 0x37u8, 0x22u8, 0x25u8, 0x2Cu8, 0x2Bu8,
        0x6u8, 0x1u8, 0x8u8, 0xFu8, 0x1Au8, 0x1Du8, 0x14u8, 0x13u8,

        0xAEu8, 0xA9u8, 0xA0u8, 0xA7u8, 0xB2u8, 0xB5u8, 0xBCu8, 0xBBu8,
        0x96u8, 0x91u8, 0x98u8, 0x9Fu8, 0x8Au8, 0x8Du8, 0x84u8, 0x83u8,
        0xDEu8, 0xD9u8, 0xD0u8, 0xD7u8, 0xC2u8, 0xC5u8, 0xCCu8, 0xCBu8,
        0xE6u8, 0xE1u8, 0xE8u8, 0xEFu8, 0xFAu8, 0xFDu8, 0xF4u8, 0xF3u8,
    ];
    for (i, oracle) in ORACLES.iter().enumerate() {
        let mut trial = 0u8;
        pbuf::crc8_u8(&mut trial, i as u8);
        assert_eq!(trial, *oracle);
    }
}

#[test]
fn crc8_u32() {
    const ORACLES: [(u32, u8); 20] = [
        (0x8C7F0AACu32, 0x5Cu8),
        (0x97C4AA2Fu32, 0x86u8),
        (0xB716A675u32, 0x6Cu8),
        (0xD821CCC0u32, 0x60u8),
        (0x9A4EB343u32, 0xB4u8),
        (0xDBA252FBu32, 0x46u8),
        (0x8B7D76C3u32, 0x79u8),
        (0xD8E57D67u32, 0xC5u8),
        (0x6C74A409u32, 0xB0u8),
        (0x9FA1DED3u32, 0x2Du8),
        (0xA5595115u32, 0xA5u8),
        (0x6266D6F2u32, 0xDFu8),
        (0x7005B724u32, 0x7Cu8),
        (0x4C2B3A57u32, 0x22u8),
        (0xE44B3C46u32, 0x8Au8),
        (0xE84BDD8u32, 0xCEu8),
        (0xF6B29A58u32, 0xCBu8),
        (0x45CCCD8Cu32, 0x8u8),
        (0x6229393Au32, 0xD4u8),
        (0x7A4842C1u32, 0x7Eu8),
    ];
    for (inp, oracle) in ORACLES.iter() {
        let mut trial = 0u8;
        pbuf::crc8_u32(&mut trial, *inp);
        assert_eq!(trial, *oracle);
    }
}

#[test]
fn crc8_u64() {
    const ORACLES: [(u64, u8); 20] = [
        (0x972B41F45D8CA4E2u64, 0x1Fu8),
        (0x4D922E9E0241564Du64, 0x8u8),
        (0x107E7E75F66E3959u64, 0xA2u8),
        (0x272A86822092CC5Cu64, 0x99u8),
        (0x910B13599B63BD09u64, 0x33u8),
        (0x48951A122FBD438Eu64, 0x80u8),
        (0xEFC3EFDED3991823u64, 0x6Cu8),
        (0x7D7CEEA731573D83u64, 0xEEu8),
        (0xDA07215681A13CEu64, 0xB1u8),
        (0xF342B3DFC68E87Eu64, 0x16u8),
        (0xB5D9D9A1746E49BAu64, 0x34u8),
        (0xF916885B8A64BA5Eu64, 0xF5u8),
        (0x59D51A2380F2D4C0u64, 0x52u8),
        (0xF1BBD44AC4556455u64, 0x12u8),
        (0x5FB7DDE6B31B59D5u64, 0x75u8),
        (0x2E5DA9B1F3FF6819u64, 0x61u8),
        (0xB97D0F8C74A7CB1Du64, 0x59u8),
        (0x6D4F3A9FFA523DD6u64, 0x1Cu8),
        (0xEAD846D200E45402u64, 0x50u8),
        (0x6AE9989574F55F5Au64, 0x96u8),
    ];
    for (inp, oracle) in ORACLES.iter() {
        let mut trial = 0u8;
        pbuf::crc8_u64(&mut trial, *inp);
        assert_eq!(trial, *oracle);
    }
}

#[test]
fn crc8_blob() {
    const ORACLES: [(&'static [u8], u8); 20] = [
        (b"eefbf", 0x8u8),
        (b"checbe", 0x3u8),
        (b"fbaa", 0xF2u8),
        (b"degffdhaedf", 0xEBu8),
        (b"f", 0x35u8),
        (b"cahehfgc", 0x12u8),
        (b"a", 0x20u8),
        (b"ga", 0xBEu8),
        (b"fbeedch", 0xD6u8),
        (b"bfghghgecfffcfff", 0xACu8),
        (b"d", 0x3Bu8),
        (b"ebdbecdaadbecfabdchf", 0x6Cu8),
        (b"aeffbceafgeddfdhbhcegadegb", 0x3Bu8),
        (b"bg", 0xEDu8),
        (b"", 0x0u8),
        (b"ddecgdeab", 0x36u8),
        (b"daeeeagadcgcedhdfhhdbdah", 0xC2u8),
        (b"ebcaebgdfffe", 0x4u8),
        (b"egcafehabchgeceecdhffah", 0x15u8),
        (b"bcagagdcfedfehgc", 0x32u8),
    ];
    for (inp, oracle) in ORACLES.iter() {
        let mut trial = 0u8;
        pbuf::crc8_blob(&mut trial, inp);
        assert_eq!(trial, *oracle);
    }
}
