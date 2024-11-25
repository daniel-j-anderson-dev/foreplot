use anyhow::{anyhow, Error};
use std::{fmt::Display, str::FromStr};

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherForecastOfficeId {
    AKQ, ALY, BGM, BOX, BTV, BUF, CAE, CAR, CHS, CLE, CTP, GSP, GYX, ILM, ILN,
    LWX, MHX, OKX, PBZ, PHI, RAH, RLX, RNK, ABQ, AMA, BMX, BRO, CRP, EPZ, EWX,
    FFC, FWD, HGX, HUN, JAN, JAX, KEY, LCH, LIX, LUB, LZK, MAF, MEG, MFL, MLB,
    MOB, MRX, OHX, OUN, SHV, SJT, SJU, TAE, TBW, TSA, ABR, APX, ARX, BIS, BOU,
    CYS, DDC, DLH, DMX, DTX, DVN, EAX, FGF, FSD, GID, GJT, GLD, GRB, GRR, ICT,
    ILX, IND, IWX, JKL, LBF, LMK, LOT, LSX, MKX, MPX, MQT, OAX, PAH, PUB, RIW,
    SGF, TOP, UNR, BOI, BYZ, EKA, FGZ, GGW, HNX, LKN, LOX, MFR, MSO, MTR, OTX,
    PDT, PIH, PQR, PSR, REV, SEW, SGX, SLC, STO, TFX, TWC, VEF, AER, AFC, AFG,
    AJK, ALU, GUM, HPA, HFO, PPG, STU, NH1, NH2, ONA, ONP,
}
impl FromStr for WeatherForecastOfficeId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[rustfmt::skip]
        let output = match s.to_uppercase().as_str() {
            "AKQ" => Self::AKQ, "ALY" => Self::ALY, "BGM" => Self::BGM, "BOX" => Self::BOX,
            "BTV" => Self::BTV, "BUF" => Self::BUF, "CAE" => Self::CAE, "CAR" => Self::CAR,
            "CHS" => Self::CHS, "CLE" => Self::CLE, "CTP" => Self::CTP, "GSP" => Self::GSP,
            "GYX" => Self::GYX, "ILM" => Self::ILM, "ILN" => Self::ILN, "LWX" => Self::LWX,
            "MHX" => Self::MHX, "OKX" => Self::OKX, "PBZ" => Self::PBZ, "PHI" => Self::PHI,
            "RAH" => Self::RAH, "RLX" => Self::RLX, "RNK" => Self::RNK, "ABQ" => Self::ABQ,
            "AMA" => Self::AMA, "BMX" => Self::BMX, "BRO" => Self::BRO, "CRP" => Self::CRP,
            "EPZ" => Self::EPZ, "EWX" => Self::EWX, "FFC" => Self::FFC, "FWD" => Self::FWD,
            "HGX" => Self::HGX, "HUN" => Self::HUN, "JAN" => Self::JAN, "JAX" => Self::JAX,
            "KEY" => Self::KEY, "LCH" => Self::LCH, "LIX" => Self::LIX, "LUB" => Self::LUB,
            "LZK" => Self::LZK, "MAF" => Self::MAF, "MEG" => Self::MEG, "MFL" => Self::MFL,
            "MLB" => Self::MLB, "MOB" => Self::MOB, "MRX" => Self::MRX, "OHX" => Self::OHX,
            "OUN" => Self::OUN, "SHV" => Self::SHV, "SJT" => Self::SJT, "SJU" => Self::SJU,
            "TAE" => Self::TAE, "TBW" => Self::TBW, "TSA" => Self::TSA, "ABR" => Self::ABR,
            "APX" => Self::APX, "ARX" => Self::ARX, "BIS" => Self::BIS, "BOU" => Self::BOU,
            "CYS" => Self::CYS, "DDC" => Self::DDC, "DLH" => Self::DLH, "DMX" => Self::DMX,
            "DTX" => Self::DTX, "DVN" => Self::DVN, "EAX" => Self::EAX, "FGF" => Self::FGF,
            "FSD" => Self::FSD, "GID" => Self::GID, "GJT" => Self::GJT, "GLD" => Self::GLD,
            "GRB" => Self::GRB, "GRR" => Self::GRR, "ICT" => Self::ICT, "ILX" => Self::ILX,
            "IND" => Self::IND, "IWX" => Self::IWX, "JKL" => Self::JKL, "LBF" => Self::LBF,
            "LMK" => Self::LMK, "LOT" => Self::LOT, "LSX" => Self::LSX, "MKX" => Self::MKX,
            "MPX" => Self::MPX, "MQT" => Self::MQT, "OAX" => Self::OAX, "PAH" => Self::PAH,
            "PUB" => Self::PUB, "RIW" => Self::RIW, "SGF" => Self::SGF, "TOP" => Self::TOP,
            "UNR" => Self::UNR, "BOI" => Self::BOI, "BYZ" => Self::BYZ, "EKA" => Self::EKA,
            "FGZ" => Self::FGZ, "GGW" => Self::GGW, "HNX" => Self::HNX, "LKN" => Self::LKN,
            "LOX" => Self::LOX, "MFR" => Self::MFR, "MSO" => Self::MSO, "MTR" => Self::MTR,
            "OTX" => Self::OTX, "PDT" => Self::PDT, "PIH" => Self::PIH, "PQR" => Self::PQR,
            "PSR" => Self::PSR, "REV" => Self::REV, "SEW" => Self::SEW, "SGX" => Self::SGX,
            "SLC" => Self::SLC, "STO" => Self::STO, "TFX" => Self::TFX, "TWC" => Self::TWC,
            "VEF" => Self::VEF, "AER" => Self::AER, "AFC" => Self::AFC, "AFG" => Self::AFG,
            "AJK" => Self::AJK, "ALU" => Self::ALU, "GUM" => Self::GUM, "HPA" => Self::HPA,
            "HFO" => Self::HFO, "PPG" => Self::PPG, "STU" => Self::STU, "NH1" => Self::NH1,
            "NH2" => Self::NH2, "ONA" => Self::ONA, "ONP" => Self::ONP,
            _ => return Err(anyhow!("{s} is not one a valid Weather forecast office call sign")),
        };
        Ok(output)
    }
}
impl Display for WeatherForecastOfficeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[rustfmt::skip]
        let s = match self {
            Self::AKQ => "AKQ", Self::ALY => "ALY", Self::BGM => "BGM", Self::BOX => "BOX",
            Self::BTV => "BTV", Self::BUF => "BUF", Self::CAE => "CAE", Self::CAR => "CAR",
            Self::CHS => "CHS", Self::CLE => "CLE", Self::CTP => "CTP", Self::GSP => "GSP",
            Self::GYX => "GYX", Self::ILM => "ILM", Self::ILN => "ILN", Self::LWX => "LWX",
            Self::MHX => "MHX", Self::OKX => "OKX", Self::PBZ => "PBZ", Self::PHI => "PHI",
            Self::RAH => "RAH", Self::RLX => "RLX", Self::RNK => "RNK", Self::ABQ => "ABQ",
            Self::AMA => "AMA", Self::BMX => "BMX", Self::BRO => "BRO", Self::CRP => "CRP",
            Self::EPZ => "EPZ", Self::EWX => "EWX", Self::FFC => "FFC", Self::FWD => "FWD",
            Self::HGX => "HGX", Self::HUN => "HUN", Self::JAN => "JAN", Self::JAX => "JAX",
            Self::KEY => "KEY", Self::LCH => "LCH", Self::LIX => "LIX", Self::LUB => "LUB",
            Self::LZK => "LZK", Self::MAF => "MAF", Self::MEG => "MEG", Self::MFL => "MFL",
            Self::MLB => "MLB", Self::MOB => "MOB", Self::MRX => "MRX", Self::OHX => "OHX",
            Self::OUN => "OUN", Self::SHV => "SHV", Self::SJT => "SJT", Self::SJU => "SJU",
            Self::TAE => "TAE", Self::TBW => "TBW", Self::TSA => "TSA", Self::ABR => "ABR",
            Self::APX => "APX", Self::ARX => "ARX", Self::BIS => "BIS", Self::BOU => "BOU",
            Self::CYS => "CYS", Self::DDC => "DDC", Self::DLH => "DLH", Self::DMX => "DMX",
            Self::DTX => "DTX", Self::DVN => "DVN", Self::EAX => "EAX", Self::FGF => "FGF",
            Self::FSD => "FSD", Self::GID => "GID", Self::GJT => "GJT", Self::GLD => "GLD",
            Self::GRB => "GRB", Self::GRR => "GRR", Self::ICT => "ICT", Self::ILX => "ILX",
            Self::IND => "IND", Self::IWX => "IWX", Self::JKL => "JKL", Self::LBF => "LBF",
            Self::LMK => "LMK", Self::LOT => "LOT", Self::LSX => "LSX", Self::MKX => "MKX",
            Self::MPX => "MPX", Self::MQT => "MQT", Self::OAX => "OAX", Self::PAH => "PAH",
            Self::PUB => "PUB", Self::RIW => "RIW", Self::SGF => "SGF", Self::TOP => "TOP",
            Self::UNR => "UNR", Self::BOI => "BOI", Self::BYZ => "BYZ", Self::EKA => "EKA",
            Self::FGZ => "FGZ", Self::GGW => "GGW", Self::HNX => "HNX", Self::LKN => "LKN",
            Self::LOX => "LOX", Self::MFR => "MFR", Self::MSO => "MSO", Self::MTR => "MTR",
            Self::OTX => "OTX", Self::PDT => "PDT", Self::PIH => "PIH", Self::PQR => "PQR",
            Self::PSR => "PSR", Self::REV => "REV", Self::SEW => "SEW", Self::SGX => "SGX",
            Self::SLC => "SLC", Self::STO => "STO", Self::TFX => "TFX", Self::TWC => "TWC",
            Self::VEF => "VEF", Self::AER => "AER", Self::AFC => "AFC", Self::AFG => "AFG",
            Self::AJK => "AJK", Self::ALU => "ALU", Self::GUM => "GUM", Self::HPA => "HPA",
            Self::HFO => "HFO", Self::PPG => "PPG", Self::STU => "STU", Self::NH1 => "NH1",
            Self::NH2 => "NH2", Self::ONA => "ONA", Self::ONP => "ONP",
        };
        write!(f, "{s}")
    }
}
