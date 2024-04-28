use std::fmt::{self, Display};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    NCR,
    CAR,
    I,
    II,
    III,
    IVA,
    IVB,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
    XI,
    XII,
    XIII,
    BARMM,
}

impl Region {
    pub fn from_str(s: &str) -> Option<Self> {
        let normalized = s
            .trim()
            .to_lowercase()
            .replace(" ", "_")
            .replace("(", "")
            .replace(")", "");
        match normalized.as_str() {
            "ncr" | "national_capital_region" | "ncr_national_capital_region" => Some(Region::NCR),
            "car" | "cordillera_administrative_region" | "car_cordillera_administrative_region" => {
                Some(Region::CAR)
            }
            "i" | "ilocos_region" | "region_i_ilocos_region" | "region_i" | "1" | "region_1" => {
                Some(Region::I)
            }
            "ii"
            | "cagayan_valley"
            | "region_ii_cagayan_valley"
            | "region_ii"
            | "2"
            | "region_2" => Some(Region::II),
            "iii"
            | "central_luzon"
            | "region_iii_central_luzon"
            | "region_iii"
            | "3"
            | "region_3" => Some(Region::III),
            "iva" | "calabarzon" | "region_iva_calabarzon" | "region_iva" | "4" | "region_4" => {
                Some(Region::IVA)
            }
            "ivb" | "mimaropa" | "region_ivb_mimaropa" | "region_ivb" | "4b" | "region_4b" => {
                Some(Region::IVB)
            }
            "v" | "bicol_region" | "region_v_bicol_region" | "region_v" | "5" | "region_5" => {
                Some(Region::V)
            }
            "vi"
            | "western_visayas"
            | "region_vi_western_visayas"
            | "region_vi"
            | "6"
            | "region_6" => Some(Region::VI),
            "vii"
            | "central_visayas"
            | "region_vii_central_visayas"
            | "region_vii"
            | "7"
            | "region_7" => Some(Region::VII),
            "viii"
            | "eastern_visayas"
            | "region_viii_eastern_visayas"
            | "region_viii"
            | "8"
            | "region_8" => Some(Region::VIII),
            "ix"
            | "zamboanga_peninsula"
            | "region_ix_zamboanga_peninsula"
            | "region_ix"
            | "9"
            | "region_9" => Some(Region::IX),
            "x"
            | "northern_mindanao"
            | "region_x_northern_mindanao"
            | "region_x"
            | "10"
            | "region_10" => Some(Region::X),
            "xi" | "davao_region" | "region_xi_davao_region" | "region_xi" | "11" | "region_11" => {
                Some(Region::XI)
            }
            "xii"
            | "soccsksargen"
            | "region_xii_soccsksargen"
            | "region_xii"
            | "12"
            | "region_12" => Some(Region::XII),
            "xiii"
            | "caraga_region"
            | "region_xiii_caraga_region"
            | "region_xiii"
            | "13"
            | "region_13" => Some(Region::XIII),
            "barmm"
            | "bangsamoro_autonomous_region_in_muslim_mindanao"
            | "barmm_bangsamoro_autonomous_region_in_muslim_mindanao" => Some(Region::BARMM),
            _ => None,
        }
    }

    pub fn iter() -> impl Iterator<Item = Region> {
        [
            Region::NCR,
            Region::CAR,
            Region::I,
            Region::II,
            Region::III,
            Region::IVA,
            Region::IVB,
            Region::V,
            Region::VI,
            Region::VII,
            Region::VIII,
            Region::IX,
            Region::X,
            Region::XI,
            Region::XII,
            Region::XIII,
            Region::BARMM,
        ]
        .iter()
        .copied()
    }

    pub fn abbrev(&self) -> &'static str {
        match *self {
            Region::NCR => "NCR",
            Region::CAR => "CAR",
            Region::I => "Region I",
            Region::II => "Region II",
            Region::III => "Region III",
            Region::IVA => "Region IV-A",
            Region::IVB => "Region IV-B",
            Region::V => "Region V",
            Region::VI => "Region VI",
            Region::VII => "Region VII",
            Region::VIII => "Region VIII",
            Region::IX => "Region IX",
            Region::X => "Region X",
            Region::XI => "Region XI",
            Region::XII => "Region XII",
            Region::XIII => "Region XIII",
            Region::BARMM => "BARMM",
        }
    }

    pub fn code(&self) -> &'static str {
        match *self {
            Region::NCR => "ncr",
            Region::CAR => "car",
            Region::I => "1",
            Region::II => "2",
            Region::III => "3",
            Region::IVA => "4a",
            Region::IVB => "4b",
            Region::V => "5",
            Region::VI => "6",
            Region::VII => "7",
            Region::VIII => "8",
            Region::IX => "9",
            Region::X => "10",
            Region::XI => "11",
            Region::XII => "12",
            Region::XIII => "13",
            Region::BARMM => "barmm",
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Region::NCR => "National Capital Region",
            Region::CAR => "Cordillera Administrative Region",
            Region::I => "Ilocos Region",
            Region::II => "Cagayan Valley",
            Region::III => "Central Luzon",
            Region::IVA => "CALABARZON",
            Region::IVB => "MIMAROPA",
            Region::V => "Bicol Region",
            Region::VI => "Western Visayas",
            Region::VII => "Central Visayas",
            Region::VIII => "Eastern Visayas",
            Region::IX => "Zamboanga Peninsula",
            Region::X => "Northern Mindanao",
            Region::XI => "Davao Region",
            Region::XII => "SOCCSKSARGEN",
            Region::XIII => "Caraga Region",
            Region::BARMM => "Bangsamoro Autonomous Region in Muslim Mindanao",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match *self {
            Region::NCR => "(NCR) National Capital Region",
            Region::CAR => "(CAR) Cordillera Administrative Region",
            Region::I => "(Region I) Ilocos Region",
            Region::II => "(Region II) Cagayan Valley",
            Region::III => "(Region III) Central Luzon",
            Region::IVA => "(Region IVA) CALABARZON",
            Region::IVB => "(Region IVB) MIMAROPA",
            Region::V => "(Region V) Bicol Region",
            Region::VI => "(Region VI) Western Visayas",
            Region::VII => "(Region VII) Central Visayas",
            Region::VIII => "(Region VIII) Eastern Visayas",
            Region::IX => "(Region IX) Zamboanga Peninsula",
            Region::X => "(Region X) Northern Mindanao",
            Region::XI => "(Region XI) Davao Region",
            Region::XII => "(Region XII) SOCCSKSARGEN",
            Region::XIII => "(Region XIII) Caraga Region",
            Region::BARMM => "(BARMM) Bangsamoro Autonomous Region in Muslim Mindanao",
        }
    }

    pub fn keys() -> Vec<&'static str> {
        Region::iter().map(|r| r.code()).collect()
    }

    pub fn codes() -> Vec<&'static str> {
        Region::iter().map(|r| r.abbrev()).collect()
    }

    pub fn names() -> Vec<&'static str> {
        Region::iter().map(|r| r.name()).collect()
    }

    pub fn list_by_full_name() -> HashMap<&'static str, &'static str> {
        Region::iter().map(|r| (r.code(), r.full_name())).collect()
    }

    pub fn list() -> HashMap<&'static str, &'static str> {
        Region::iter().map(|r| (r.code(), r.name())).collect()
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) {}", self.abbrev(), self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Region::from_str("ncr"), Some(Region::NCR));
        assert_eq!(Region::from_str("CAR"), Some(Region::CAR));
        assert_eq!(Region::from_str("1"), Some(Region::I));
        assert_eq!(Region::from_str("region_ii"), Some(Region::II));
        assert_eq!(Region::from_str("3"), Some(Region::III));
        assert_eq!(Region::from_str("region_iva"), Some(Region::IVA));
        assert_eq!(Region::from_str("region ivb"), Some(Region::IVB));
        assert_eq!(Region::from_str("  5  "), Some(Region::V));
        assert_eq!(Region::from_str("region_vi"), Some(Region::VI));
        assert_eq!(Region::from_str("Region VII"), Some(Region::VII));
        assert_eq!(Region::from_str("8"), Some(Region::VIII));
        assert_eq!(Region::from_str("IX"), Some(Region::IX));
        assert_eq!(Region::from_str("10"), Some(Region::X));
        assert_eq!(Region::from_str("11"), Some(Region::XI));
        assert_eq!(Region::from_str("12"), Some(Region::XII));
        assert_eq!(Region::from_str("13"), Some(Region::XIII));
        assert_eq!(Region::from_str("barmm"), Some(Region::BARMM));
        assert_eq!(Region::from_str("invalid"), None);
    }

    #[test]
    fn test_iter() {
        let mut regions = Region::iter();
        assert_eq!(regions.next(), Some(Region::NCR));
        assert_eq!(regions.count(), 16); // Test after one item was taken
    }

    #[test]
    fn test_display() {
        let ncr = Region::NCR;
        assert_eq!(format!("{}", ncr), "(NCR) National Capital Region");
    }

    #[test]
    fn test_keys() {
        let keys = Region::keys();
        assert_eq!(keys.len(), 17);
        assert!(keys.contains(&"ncr"));
        assert!(keys.contains(&"car"));
    }

    #[test]
    fn test_codes() {
        let codes = Region::codes();
        assert_eq!(codes.len(), 17);
        assert!(codes.contains(&"Region I"));
        assert!(codes.contains(&"Region II"));
    }

    #[test]
    fn test_names() {
        let names = Region::names();
        assert_eq!(names.len(), 17);
        assert!(names.contains(&"National Capital Region"));
        assert!(names.contains(&"Cordillera Administrative Region"));
    }

    #[test]
    fn test_list() {
        let list = Region::list();
        assert_eq!(list.get("ncr"), Some(&"National Capital Region"));
        assert_eq!(list.get("car"), Some(&"Cordillera Administrative Region"));
    }

    #[test]
    fn test_list_by_full_name() {
        let list = Region::list_by_full_name();
        assert_eq!(list.get("ncr"), Some(&"(NCR) National Capital Region"));
        assert_eq!(list.get("car"), Some(&"(CAR) Cordillera Administrative Region"));
    }
}
