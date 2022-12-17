use super::{pkx::Pkx, poke_crypto, types};
use no_std_io::{EndianRead, Reader};

pub type Pk9Bytes = [u8; Pk9::STORED_SIZE];

pub struct Pk9 {
    data: Pk9Bytes,
}

impl Default for Pk9 {
    fn default() -> Self {
        Self {
            data: [0; Pk9::STORED_SIZE],
        }
    }
}

impl Reader for Pk9 {
    fn get_slice(&self) -> &[u8] {
        &self.data
    }
}

impl Pkx for Pk9 {
    type StoredBytes = Pk9Bytes;
    const STORED_SIZE: usize = 328;
    const BLOCK_SIZE: usize = 80;

    fn new_pkx(data: Self::StoredBytes) -> Self {
        Self { data }
    }

    fn is_encrypted(data: &Self::StoredBytes) -> bool {
        data.default_read_le::<u16>(0x70) != 0 || data.default_read_le::<u16>(0x110) != 0
    }

    fn decrypt(data: Self::StoredBytes) -> Self::StoredBytes {
        poke_crypto::decrypt::<{ Self::STORED_SIZE }, { Self::BLOCK_SIZE }>(data)
    }

    fn encrypt(data: Self::StoredBytes) -> Self::StoredBytes {
        poke_crypto::encrypt::<{ Self::STORED_SIZE }, { Self::BLOCK_SIZE }>(data)
    }

    fn get_encrypted(&self) -> Self::StoredBytes {
        Self::encrypt(self.data)
    }

    fn encryption_constant(&self) -> u32 {
        self.default_read_le(0x00)
    }

    fn sanity(&self) -> u16 {
        self.default_read_le(0x04)
    }

    fn checksum(&self) -> u16 {
        self.default_read_le(0x06)
    }

    fn species(&self) -> types::Species {
        self.default_read_le::<u16>(0x08).into()
    }

    fn tid(&self) -> u16 {
        self.default_read_le(0x0C)
    }

    fn sid(&self) -> u16 {
        self.default_read_le(0x0E)
    }

    fn ability(&self) -> types::Ability {
        self.default_read::<u16>(0x14).into()
    }

    fn ability_number(&self) -> types::AbilityNumber {
        self.default_read::<u8>(0x16).into()
    }

    fn pid(&self) -> u32 {
        self.default_read_le(0x1C)
    }

    fn nature(&self) -> types::Nature {
        self.default_read::<u8>(0x20).into()
    }

    fn minted_nature(&self) -> types::Nature {
        self.default_read::<u8>(0x21).into()
    }

    fn gender(&self) -> types::Gender {
        let byte = self.default_read::<u8>(0x22);
        ((byte >> 1) & 3).into()
    }

    fn evs(&self) -> types::Stats {
        types::Stats {
            hp: self.default_read(0x26),
            atk: self.default_read(0x27),
            def: self.default_read(0x28),
            spe: self.default_read(0x29),
            spa: self.default_read(0x2A),
            spd: self.default_read(0x2B),
        }
    }

    fn move1(&self) -> types::Move {
        self.default_read::<u16>(0x72).into()
    }

    fn move2(&self) -> types::Move {
        self.default_read::<u16>(0x74).into()
    }

    fn move3(&self) -> types::Move {
        self.default_read::<u16>(0x76).into()
    }

    fn move4(&self) -> types::Move {
        self.default_read::<u16>(0x78).into()
    }

    fn iv32(&self) -> u32 {
        self.default_read_le(0x8C)
    }

    fn current_handler(&self) -> u8 {
        self.default_read(0xC4)
    }

    fn ht_friendship(&self) -> u8 {
        self.default_read(0xC8)
    }

    fn language(&self) -> types::Language {
        self.default_read::<u8>(0xC3).into()
    }

    fn ot_friendship(&self) -> u8 {
        self.default_read(0x112)
    }

    fn calculate_checksum(&self) -> u16 {
        poke_crypto::calculate_checksum(&self.data[8..Pk9::STORED_SIZE])
    }
}

impl From<Pk9Bytes> for Pk9 {
    fn from(data: Pk9Bytes) -> Self {
        Self::new_or_default(data)
    }
}

#[derive(EndianRead)]
pub struct Ek9 {
    data: Pk9Bytes,
}

impl Default for Ek9 {
    fn default() -> Self {
        Self {
            data: [0; Pk9::STORED_SIZE],
        }
    }
}

impl From<Ek9> for Pk9 {
    fn from(ekx: Ek9) -> Self {
        ekx.data.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_EKX: Pk9Bytes = [
        0xFE, 0x6E, 0xD5, 0xF8, 0x00, 0x00, 0xEF, 0x61, 0x6B, 0x51, 0x60, 0x16, 0x95, 0x30, 0x80,
        0x57, 0x7E, 0xD8, 0x0A, 0x2F, 0xF1, 0x83, 0x24, 0x9A, 0xFE, 0x66, 0xEE, 0xBD, 0xAE, 0xE2,
        0x09, 0xFD, 0xC4, 0x4C, 0x6A, 0xA6, 0x5B, 0x8F, 0x4D, 0xE9, 0xFC, 0x79, 0xE1, 0x0F, 0xDC,
        0xE4, 0x9E, 0xEB, 0x5C, 0x46, 0x10, 0x7D, 0x79, 0xC1, 0xB5, 0x52, 0x75, 0x5F, 0xFE, 0x72,
        0xC4, 0x1B, 0x39, 0x85, 0x41, 0xCC, 0xF2, 0x79, 0x0E, 0x47, 0x06, 0x09, 0x54, 0x3C, 0xFA,
        0xB5, 0xED, 0x23, 0x6D, 0x9B, 0x52, 0x16, 0x51, 0x5F, 0xC1, 0x28, 0xBC, 0xB0, 0xE1, 0xF9,
        0xEE, 0xD7, 0x92, 0x73, 0xBB, 0x4C, 0x3F, 0xA7, 0xA0, 0x30, 0x7B, 0xA7, 0xD9, 0x1C, 0x94,
        0x86, 0xB6, 0x4F, 0x80, 0x77, 0xB6, 0x20, 0xAA, 0x37, 0x09, 0x0C, 0x99, 0xEC, 0x62, 0x1B,
        0x1D, 0xEB, 0xD3, 0xF3, 0xDD, 0x30, 0x9B, 0x5B, 0x6F, 0x6F, 0xD2, 0xE7, 0x91, 0x8C, 0xE0,
        0x51, 0x5F, 0x2A, 0x62, 0x66, 0x41, 0x1E, 0x23, 0xE1, 0x5A, 0x71, 0x1E, 0xE0, 0x7D, 0x09,
        0xE0, 0x29, 0x30, 0xF5, 0x83, 0x17, 0x29, 0x03, 0xD5, 0x57, 0x0B, 0x35, 0xEB, 0x14, 0x72,
        0xA3, 0x73, 0xCD, 0xCF, 0x2C, 0x1C, 0xD5, 0x74, 0xDC, 0xE8, 0x7E, 0xE4, 0xB1, 0xEA, 0x6D,
        0x2A, 0x15, 0xAE, 0x33, 0x5F, 0x34, 0x3A, 0xED, 0x92, 0x7E, 0xC0, 0xF8, 0x04, 0xD8, 0x69,
        0xE9, 0x6E, 0xAB, 0x2B, 0x3D, 0x5C, 0x15, 0xC9, 0xDA, 0x46, 0xCA, 0x7F, 0x67, 0xD3, 0x5D,
        0x53, 0x8D, 0x61, 0x0D, 0x87, 0xE0, 0x6E, 0x70, 0xDB, 0x89, 0xD2, 0x1F, 0x8D, 0xDD, 0x72,
        0x57, 0x00, 0x18, 0x29, 0xA1, 0xE2, 0x66, 0x3B, 0x40, 0x47, 0x26, 0x23, 0x27, 0x99, 0x65,
        0xC6, 0xA2, 0x87, 0x6E, 0x15, 0x2E, 0x62, 0xB8, 0xE7, 0x6C, 0x9F, 0x7D, 0x72, 0x48, 0xFE,
        0x13, 0x5B, 0x4E, 0xE2, 0x7A, 0xEA, 0xE7, 0xAB, 0xB8, 0xA8, 0x2B, 0x38, 0xEA, 0xAB, 0xBF,
        0x3C, 0xFB, 0x72, 0xD2, 0x7C, 0xA6, 0x0C, 0xB5, 0x78, 0xB3, 0x82, 0x36, 0x9D, 0xDE, 0x4E,
        0x2D, 0x32, 0xE1, 0x3C, 0x64, 0x87, 0xE2, 0x87, 0xD2, 0xED, 0xD8, 0x9A, 0xC9, 0x1D, 0xEB,
        0xF3, 0xC0, 0xF3, 0xF9, 0xE3, 0x6D, 0xB8, 0x40, 0x41, 0xBE, 0x44, 0xCA, 0x64, 0x3D, 0x40,
        0x63, 0x63, 0x7F, 0xDB, 0x49, 0x03, 0x5C, 0x98, 0x1B, 0xA5, 0x77, 0x35, 0x36,
    ];

    const TEST_PKX: Pk9Bytes = [
        0xFE, 0x6E, 0xD5, 0xF8, 0x00, 0x00, 0xEF, 0x61, 0x84, 0x00, 0x18, 0x01, 0x85, 0xAD, 0x3B,
        0x87, 0xF3, 0x6F, 0x06, 0x00, 0x96, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC8, 0x99,
        0x12, 0xCB, 0x16, 0x16, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x45, 0x81,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4D, 0x00,
        0xE9, 0x00, 0x74, 0x00, 0x61, 0x00, 0x6D, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x70, 0x00, 0x68,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xB4, 0x00, 0xFF, 0xFF, 0xFF, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x08, 0x13,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x53, 0x00, 0x61, 0x00, 0x6B, 0x00, 0x75, 0x00, 0x72, 0x00, 0x61, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x02, 0x01, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xFF, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x62, 0x00, 0x79, 0x00, 0x7A,
        0x00, 0x61, 0x00, 0x62, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16,
        0x0B, 0x18, 0x4B, 0x00, 0x00, 0x48, 0x75, 0x04, 0xCB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    mod is_encrypted {
        use super::*;

        #[test]
        fn encrypted() {
            assert_eq!(Pk9::is_encrypted(&TEST_EKX), true)
        }

        #[test]
        fn decrypted() {
            assert_eq!(Pk9::is_encrypted(&TEST_PKX), false)
        }
    }

    #[test]
    fn should_decrypt() {
        let pkx = Pk9::decrypt(TEST_EKX);
        assert_eq!(pkx, TEST_PKX);
    }

    #[test]
    fn should_encrypt() {
        let ekx = Pk9::encrypt(TEST_PKX);
        assert_eq!(ekx, TEST_EKX);
    }

    #[test]
    fn should_get_encrypted() {
        let ekx = Pk9::new(TEST_PKX).get_encrypted();
        assert_eq!(ekx, TEST_EKX);
    }

    #[test]
    fn pk9_data_size_should_be_232() {
        assert_eq!(core::mem::size_of::<Pk9Bytes>(), Pk9::STORED_SIZE);
    }

    #[test]
    fn should_read_species() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.species(), types::Species::Ditto);
    }

    #[test]
    fn should_read_pid() {
        let pkx = Pk9::new(TEST_EKX);
        let pid = 0xcb1299c8;
        assert_eq!(pkx.pid(), pid)
    }

    #[test]
    fn should_read_tid() {
        let pkx = Pk9::new(TEST_EKX);
        let tid = 44421;
        assert_eq!(pkx.tid(), tid)
    }

    #[test]
    fn should_read_sid() {
        let pkx = Pk9::new(TEST_EKX);
        let sid = 34619;
        assert_eq!(pkx.sid(), sid)
    }

    #[test]
    fn should_read_tsv() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.tsv(), 683)
    }

    #[test]
    fn should_read_psv() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.psv(), 1325)
    }

    #[test]
    fn should_read_nature() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.nature(), types::Nature::Sassy)
    }

    #[test]
    fn should_read_minted_nature() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.minted_nature(), types::Nature::Sassy)
    }

    #[test]
    fn should_read_ability() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.ability(), types::Ability::Imposter)
    }

    #[test]
    fn should_read_ability_number() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.ability_number(), types::AbilityNumber::Hidden)
    }

    #[test]
    fn should_read_hidden_power() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.hidden_power(), types::HiddenPower::Dark)
    }

    #[test]
    fn should_read_language() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.language(), types::Language::English)
    }

    #[test]
    fn should_read_gender() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.gender(), types::Gender::Genderless)
    }

    #[test]
    fn should_read_move1() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.move1(), types::Move::Transform)
    }

    #[test]
    fn should_read_move2() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.move2(), types::Move::None)
    }

    #[test]
    fn should_read_move3() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.move3(), types::Move::None)
    }

    #[test]
    fn should_read_move4() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.move4(), types::Move::None)
    }

    #[test]
    fn should_read_ivs() {
        let pkx = Pk9::new(TEST_EKX);
        let stats = types::Stats {
            hp: 31,
            atk: 31,
            def: 31,
            spa: 31,
            spd: 31,
            spe: 31,
        };
        assert_eq!(pkx.ivs(), stats)
    }

    #[test]
    fn should_read_evs() {
        let pkx = Pk9::new(TEST_EKX);
        let stats = types::Stats {
            hp: 0,
            atk: 0,
            def: 0,
            spa: 0,
            spd: 0,
            spe: 0,
        };
        assert_eq!(pkx.evs(), stats)
    }

    #[test]
    fn should_read_ot_friendship() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.ot_friendship(), 50)
    }

    #[test]
    fn should_read_ht_friendship() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.ht_friendship(), 50)
    }

    #[test]
    fn should_read_is_egg() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.is_egg(), false)
    }

    #[test]
    fn should_read_current_handler() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.current_handler(), 1)
    }

    #[test]
    fn should_read_current_friendship() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.current_friendship(), 50)
    }

    #[test]
    fn should_read_sanity() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.sanity(), 0)
    }

    #[test]
    fn should_read_checksum() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.checksum(), 0x61ef)
    }

    #[test]
    fn should_calculate_checksum() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.calculate_checksum(), 0x61ef)
    }

    #[test]
    fn should_read_is_valid() {
        let pkx = Pk9::new(TEST_EKX);
        assert_eq!(pkx.is_valid(), true)
    }

    #[test]
    fn should_return_not_shiny_for_default() {
        let pkx = Pk9::default();
        assert_eq!(pkx.is_shiny(), false)
    }
}
