use super::types;

pub trait Pkx: Sized {
    fn encryption_constant(&self) -> u32;

    fn species(&self) -> types::Species;

    fn pid(&self) -> u32;

    fn tid(&self) -> u16;

    fn sid(&self) -> u16;

    fn nature(&self) -> types::Nature;

    fn ability(&self) -> types::Ability;

    fn ability_number(&self) -> types::AbilityNumber;

    fn language(&self) -> types::Language;

    fn gender(&self) -> types::Gender;

    fn iv32(&self) -> u32;

    fn move1(&self) -> types::Move;

    fn move2(&self) -> types::Move;

    fn move3(&self) -> types::Move;

    fn move4(&self) -> types::Move;

    fn evs(&self) -> types::Stats;

    fn ot_friendship(&self) -> u32;

    fn ht_friendship(&self) -> u32;

    fn tsv(&self) -> u16 {
        (self.tid() ^ self.sid()) >> 4
    }

    fn psv(&self) -> u16 {
        let pid = self.pid();
        let psv = ((pid >> 16) ^ (pid & 0xffff)) >> 4;
        psv as u16
    }

    fn is_shiny(&self) -> bool {
        self.psv() == self.tsv()
    }

    fn ivs(&self) -> types::Stats {
        let iv32 = self.iv32();
        types::Stats {
            hp: (iv32 & 0x1F) as u8,
            atk: ((iv32 >> 5) & 0x1F) as u8,
            def: ((iv32 >> 10) & 0x1F) as u8,
            spe: ((iv32 >> 15) & 0x1F) as u8,
            spa: ((iv32 >> 20) & 0x1F) as u8,
            spd: ((iv32 >> 25) & 0x1F) as u8,
        }
    }

    fn hidden_power_num(&self) -> u8 {
        let ivs = self.ivs();
        ((((ivs.hp & 1)
            + ((ivs.atk & 1) << 1)
            + ((ivs.def & 1) << 2)
            + ((ivs.spe & 1) << 3)
            + ((ivs.spa & 1) << 4)
            + ((ivs.spd & 1) << 5)) as u16
            * 15) as u16
            / 63) as u8
    }

    fn hidden_power(&self) -> types::HiddenPower {
        self.hidden_power_num().into()
    }

    fn gender_ratio(&self) -> types::GenderRatio {
        self.species().get_gender_ratio()
    }
}
