//! Enumeration types used throughout the API.

use serde::{Deserialize, Serialize};

/// Crystal system classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrystalSystem {
    Isometric,
    Orthorhombic,
    Hexagonal,
    Trigonal,
    Tetragonal,
    Monoclinic,
    Triclinic,
    Amorphous,
    Icosahedral,
}

/// Cleavage type classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CleavageType {
    #[serde(rename = "None Observed")]
    NoneObserved,
    #[serde(rename = "Poor/Indistinct")]
    PoorIndistinct,
    #[serde(rename = "Imperfect/Fair")]
    ImperfectFair,
    #[serde(rename = "Distinct/Good")]
    DistinctGood,
    #[serde(rename = "Very Good")]
    VeryGood,
    Perfect,
}

/// Transparency classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Diapheny {
    Transparent,
    Translucent,
    Opaque,
}

/// Entry type classification for geomaterials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EntryType {
    #[serde(rename = "0")]
    Mineral = 0,
    #[serde(rename = "1")]
    Synonym = 1,
    #[serde(rename = "2")]
    Variety = 2,
    #[serde(rename = "3")]
    Mixture = 3,
    #[serde(rename = "4")]
    Series = 4,
    #[serde(rename = "5")]
    GroupList = 5,
    #[serde(rename = "6")]
    Polytype = 6,
    #[serde(rename = "7")]
    Rock = 7,
    #[serde(rename = "8")]
    Commodity = 8,
}

impl From<u8> for EntryType {
    fn from(value: u8) -> Self {
        match value {
            0 => EntryType::Mineral,
            1 => EntryType::Synonym,
            2 => EntryType::Variety,
            3 => EntryType::Mixture,
            4 => EntryType::Series,
            5 => EntryType::GroupList,
            6 => EntryType::Polytype,
            7 => EntryType::Rock,
            8 => EntryType::Commodity,
            _ => EntryType::Mineral,
        }
    }
}

/// Fracture type classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FractureType {
    #[serde(rename = "None observed")]
    NoneObserved,
    #[serde(rename = "Irregular/Uneven")]
    IrregularUneven,
    Splintery,
    Hackly,
    Conchoidal,
    #[serde(rename = "Sub-Conchoidal")]
    SubConchoidal,
    Fibrous,
    Micaceous,
    #[serde(rename = "Step-Like")]
    StepLike,
}

/// Lustre type classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LustreType {
    Adamantine,
    #[serde(rename = "Sub-Adamantine")]
    SubAdamantine,
    Vitreous,
    #[serde(rename = "Sub-Vitreous")]
    SubVitreous,
    Resinous,
    Waxy,
    Greasy,
    Silky,
    Pearly,
    Metallic,
    #[serde(rename = "Sub-Metallic")]
    SubMetallic,
    Dull,
    Earthy,
}

/// Tenacity classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tenacity {
    Brittle,
    #[serde(rename = "very brittle")]
    VeryBrittle,
    Sectile,
    Waxy,
    Flexible,
    Elastic,
    Fragile,
    Malleable,
}

/// Optical type classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpticalType {
    Isotropic,
    Uniaxial,
    Biaxial,
}

/// Optical sign classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpticalSign {
    #[serde(rename = "+")]
    Positive,
    #[serde(rename = "-")]
    Negative,
    #[serde(rename = "+/-")]
    Both,
}

/// Optical pleochroism classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpticalPleochroism {
    #[serde(rename = "Non-pleochroic")]
    NonPleochroic,
    #[serde(rename = "Not Visible")]
    NotVisible,
    Weak,
    Visible,
    Strong,
}

/// Optical tropic classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpticalTropic {
    Isotropic,
    Anisotropic,
}

/// Magnetism classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Magnetism {
    #[serde(rename = "Non-Magnetic")]
    NonMagnetic,
    Diamagnetic,
    Paramagnetic,
    Ferromagnetic,
    Antiferromagnetic,
    Ferrimagnetic,
}

/// IMA status for minerals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImaStatus {
    Approved,
    Discredited,
    PendingPublication,
    Grandfathered,
    Questionable,
}

/// IMA notes for minerals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImaNotes {
    Rejected,
    PendingApproval,
    Group,
    Redefined,
    Renamed,
    Intermediate,
    PublishedWithoutApproval,
    UnnamedValid,
    UnnamedInvalid,
    NamedAmphibole,
}

/// Ordering options for geomaterials queries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeomaterialsOrdering {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "-id")]
    IdDesc,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "-name")]
    NameDesc,
    #[serde(rename = "updttime")]
    UpdateTime,
    #[serde(rename = "-updttime")]
    UpdateTimeDesc,
    #[serde(rename = "approval_year")]
    ApprovalYear,
    #[serde(rename = "-approval_year")]
    ApprovalYearDesc,
    #[serde(rename = "weighting")]
    Weighting,
    #[serde(rename = "-weighting")]
    WeightingDesc,
    #[serde(rename = "minstats__ms_locentries")]
    LocalityEntries,
    #[serde(rename = "-minstats__ms_locentries")]
    LocalityEntriesDesc,
    #[serde(rename = "minstats__ms_photos")]
    Photos,
    #[serde(rename = "-minstats__ms_photos")]
    PhotosDesc,
}

impl std::fmt::Display for GeomaterialsOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Id => "id",
            Self::IdDesc => "-id",
            Self::Name => "name",
            Self::NameDesc => "-name",
            Self::UpdateTime => "updttime",
            Self::UpdateTimeDesc => "-updttime",
            Self::ApprovalYear => "approval_year",
            Self::ApprovalYearDesc => "-approval_year",
            Self::Weighting => "weighting",
            Self::WeightingDesc => "-weighting",
            Self::LocalityEntries => "minstats__ms_locentries",
            Self::LocalityEntriesDesc => "-minstats__ms_locentries",
            Self::Photos => "minstats__ms_photos",
            Self::PhotosDesc => "-minstats__ms_photos",
        };
        write!(f, "{}", s)
    }
}
