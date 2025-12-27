//! Geomaterial types for the Mindat API.

use serde::{Deserialize, Serialize};

use super::common::{MinStats, Relation};
use super::enums::*;
use super::serde_helpers::{
    deserialize_optional_f64, deserialize_optional_i32, deserialize_optional_u32,
    deserialize_optional_vec, deserialize_optional_vec_i32, deserialize_optional_vec_string,
};

/// A geomaterial (mineral, variety, synonym, rock, etc.) from the Mindat database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geomaterial {
    /// Mindat ID.
    pub id: i32,
    /// Long ID string.
    #[serde(default)]
    pub longid: Option<String>,
    /// GUID.
    #[serde(default)]
    pub guid: Option<String>,
    /// Name of the geomaterial.
    pub name: Option<String>,
    /// Last update time.
    #[serde(default)]
    pub updttime: Option<String>,
    /// Mindat chemical formula.
    #[serde(default)]
    pub mindat_formula: Option<String>,
    /// Notes on the Mindat formula.
    #[serde(default)]
    pub mindat_formula_note: Option<String>,
    /// IMA-approved chemical formula.
    #[serde(default)]
    pub ima_formula: Option<String>,
    /// IMA status values.
    #[serde(default, deserialize_with = "deserialize_optional_vec_string")]
    pub ima_status: Option<Vec<String>>,
    /// IMA notes.
    #[serde(default, deserialize_with = "deserialize_optional_vec_string")]
    pub ima_notes: Option<Vec<String>>,
    /// Variety of (geomaterial ID).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub varietyof: Option<i32>,
    /// Synonym of (geomaterial ID).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub synid: Option<i32>,
    /// Polytype of (geomaterial ID).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub polytypeof: Option<i32>,
    /// Group ID (member of).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub groupid: Option<i32>,
    /// Entry type (0=mineral, 1=synonym, 2=variety, etc.).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub entrytype: Option<i32>,
    /// Entry type as text.
    #[serde(default)]
    pub entrytype_text: Option<String>,
    /// Short description.
    #[serde(default)]
    pub description_short: Option<String>,
    /// Common impurities.
    #[serde(default)]
    pub impurities: Option<String>,
    /// Elements present.
    #[serde(default, deserialize_with = "deserialize_optional_vec_string")]
    pub elements: Option<Vec<String>>,
    /// Significant elements.
    #[serde(default, deserialize_with = "deserialize_optional_vec_string")]
    pub sigelements: Option<Vec<String>>,
    /// Key elements (important for mining).
    #[serde(default, deserialize_with = "deserialize_optional_vec_string")]
    pub key_elements: Option<Vec<String>>,
    /// Type locality form.
    #[serde(default)]
    pub tlform: Option<String>,
    /// HEY index.
    #[serde(default)]
    pub cim: Option<String>,
    /// Type locality occurrence.
    #[serde(default)]
    pub occurrence: Option<String>,
    /// Other occurrences.
    #[serde(default)]
    pub otheroccurrence: Option<String>,
    /// Industrial uses.
    #[serde(default)]
    pub industrial: Option<String>,
    /// Discovery year.
    #[serde(default)]
    pub discovery_year: Option<String>,
    /// Approval year.
    #[serde(default, deserialize_with = "deserialize_optional_u32")]
    pub approval_year: Option<u32>,
    /// Publication year.
    #[serde(default, deserialize_with = "deserialize_optional_u32")]
    pub publication_year: Option<u32>,
    /// IMA history.
    #[serde(default)]
    pub ima_history: Option<String>,
    /// Transparency (diapheny).
    #[serde(default)]
    pub diapheny: Option<String>,
    /// Cleavage description.
    #[serde(default)]
    pub cleavage: Option<String>,
    /// Cleavage type.
    #[serde(default)]
    pub cleavagetype: Option<String>,
    /// Parting.
    #[serde(default)]
    pub parting: Option<String>,
    /// Tenacity.
    #[serde(default)]
    pub tenacity: Option<String>,
    /// Colour description.
    #[serde(default)]
    pub colour: Option<String>,
    /// Metamict flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub csmetamict: Option<i32>,
    /// Optical extinction direction.
    #[serde(default)]
    pub opticalextinction: Option<String>,
    /// Minimum Mohs hardness.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub hmin: Option<f64>,
    /// Maximum Mohs hardness.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub hmax: Option<f64>,
    /// Hardness type.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub hardtype: Option<i32>,
    /// Vickers hardness minimum.
    #[serde(default)]
    pub vhnmin: Option<String>,
    /// Vickers hardness maximum.
    #[serde(default)]
    pub vhnmax: Option<String>,
    /// Vickers hardness error.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub vhnerror: Option<i32>,
    /// Vickers hardness weight.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub vhng: Option<i32>,
    /// Vickers hardness time.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub vhns: Option<i32>,
    /// Luminescence.
    #[serde(default)]
    pub luminescence: Option<String>,
    /// Lustre description.
    #[serde(default)]
    pub lustre: Option<String>,
    /// Lustre type.
    #[serde(default)]
    pub lustretype: Option<String>,
    /// About name reference.
    #[serde(default)]
    pub aboutname: Option<String>,
    /// Other information.
    #[serde(default)]
    pub other: Option<String>,
    /// Streak colour.
    #[serde(default)]
    pub streak: Option<String>,
    /// Crystal system.
    #[serde(default)]
    pub csystem: Option<String>,
    /// Crystal class (point group ID).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub cclass: Option<i32>,
    /// Space group ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub spacegroup: Option<i32>,
    /// Space group setting.
    #[serde(default)]
    pub spacegroupset: Option<String>,
    /// Unit cell a.
    #[serde(default)]
    pub a: Option<String>,
    /// Unit cell b.
    #[serde(default)]
    pub b: Option<String>,
    /// Unit cell c.
    #[serde(default)]
    pub c: Option<String>,
    /// Unit cell alpha.
    #[serde(default)]
    pub alpha: Option<String>,
    /// Unit cell beta.
    #[serde(default)]
    pub beta: Option<String>,
    /// Unit cell gamma.
    #[serde(default)]
    pub gamma: Option<String>,
    /// Unit cell volume.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub va3: Option<f64>,
    /// Z value.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub z: Option<i32>,
    /// Measured density minimum.
    #[serde(default)]
    pub dmeas: Option<String>,
    /// Measured density maximum.
    #[serde(default)]
    pub dmeas2: Option<String>,
    /// Calculated density.
    #[serde(default)]
    pub dcalc: Option<String>,
    /// Fracture type.
    #[serde(default)]
    pub fracturetype: Option<String>,
    /// Morphology.
    #[serde(default)]
    pub morphology: Option<String>,
    /// Twinning.
    #[serde(default)]
    pub twinning: Option<String>,
    /// Epitaxy description.
    #[serde(default)]
    pub epitaxidescription: Option<String>,
    /// Optical type.
    #[serde(default)]
    pub opticaltype: Option<String>,
    /// Optical sign.
    #[serde(default)]
    pub opticalsign: Option<String>,
    /// Refractive index alpha.
    #[serde(default)]
    pub opticalalpha: Option<String>,
    /// Refractive index beta.
    #[serde(default)]
    pub opticalbeta: Option<String>,
    /// Refractive index gamma.
    #[serde(default)]
    pub opticalgamma: Option<String>,
    /// Refractive index omega.
    #[serde(default)]
    pub opticalomega: Option<String>,
    /// Refractive index epsilon.
    #[serde(default)]
    pub opticalepsilon: Option<String>,
    /// Refractive index n.
    #[serde(default)]
    pub opticaln: Option<String>,
    /// 2V calculated.
    #[serde(default)]
    pub optical2vcalc: Option<String>,
    /// 2V measured.
    #[serde(default)]
    pub optical2vmeasured: Option<String>,
    /// Optical dispersion.
    #[serde(default)]
    pub opticaldispersion: Option<String>,
    /// Pleochroism.
    #[serde(default)]
    pub opticalpleochroism: Option<String>,
    /// Pleochroism description.
    #[serde(default)]
    pub opticalpleochorismdesc: Option<String>,
    /// Birefringence.
    #[serde(default)]
    pub opticalbirefringence: Option<String>,
    /// Optical comments.
    #[serde(default)]
    pub opticalcomments: Option<String>,
    /// Colour in reflected light.
    #[serde(default)]
    pub opticalcolour: Option<String>,
    /// Internal reflections.
    #[serde(default)]
    pub opticalinternal: Option<String>,
    /// Optical tropic.
    #[serde(default)]
    pub opticaltropic: Option<String>,
    /// Anisotropism.
    #[serde(default)]
    pub opticalanisotropism: Option<String>,
    /// Bireflectance.
    #[serde(default)]
    pub opticalbireflectance: Option<String>,
    /// Optical reflectivity.
    #[serde(default)]
    pub opticalr: Option<String>,
    /// Refractive index minimum.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub rimin: Option<f64>,
    /// Refractive index maximum.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub rimax: Option<f64>,
    /// UV fluorescence.
    #[serde(default)]
    pub uv: Option<String>,
    /// IR spectrum.
    #[serde(default)]
    pub ir: Option<String>,
    /// Magnetism.
    #[serde(default)]
    pub magnetism: Option<String>,
    /// Type specimen storage location.
    #[serde(default)]
    pub type_specimen_store: Option<String>,
    /// IMA shortcode.
    #[serde(default)]
    pub shortcode_ima: Option<String>,
    /// Strunz classification (10th ed) parts.
    #[serde(default)]
    pub strunz10ed1: Option<String>,
    #[serde(default)]
    pub strunz10ed2: Option<String>,
    #[serde(default)]
    pub strunz10ed3: Option<String>,
    #[serde(default)]
    pub strunz10ed4: Option<String>,
    /// Dana classification (8th ed) parts.
    #[serde(default)]
    pub dana8ed1: Option<String>,
    #[serde(default)]
    pub dana8ed2: Option<String>,
    #[serde(default)]
    pub dana8ed3: Option<String>,
    #[serde(default)]
    pub dana8ed4: Option<String>,
    /// Thermal behaviour.
    #[serde(default)]
    pub thermalbehaviour: Option<String>,
    /// Electrical properties.
    #[serde(default)]
    pub electrical: Option<String>,
    /// Rock parent ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rock_parent: Option<i32>,
    /// Rock parent 2 ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rock_parent2: Option<i32>,
    /// Rock root ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rock_root: Option<i32>,
    /// Rock BGS code.
    #[serde(default)]
    pub rock_bgs_code: Option<String>,
    /// Meteoritical code.
    #[serde(default)]
    pub meteoritical_code: Option<String>,
    /// Weighting.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub weighting: Option<i32>,
    /// Relations to other geomaterials.
    #[serde(default, deserialize_with = "deserialize_optional_vec")]
    pub relations: Option<Vec<Relation>>,
    /// Mineral statistics.
    #[serde(default)]
    pub minstats: Option<MinStats>,
    /// Localities where found.
    #[serde(default, deserialize_with = "deserialize_optional_vec_i32")]
    pub locality: Option<Vec<i32>>,
    /// Type localities.
    #[serde(default, deserialize_with = "deserialize_optional_vec_i32")]
    pub type_localities: Option<Vec<i32>>,
}

/// Builder for geomaterial query parameters.
#[derive(Debug, Clone, Default)]
pub struct GeomaterialsQuery {
    /// Name filter (supports wildcards * and _).
    pub name: Option<String>,
    /// Search query.
    pub q: Option<String>,
    /// IMA approved only.
    pub ima: Option<bool>,
    /// IMA status filter.
    pub ima_status: Option<Vec<ImaStatus>>,
    /// IMA notes filter.
    pub ima_notes: Option<Vec<ImaNotes>>,
    /// Entry types filter.
    pub entrytype: Option<Vec<u8>>,
    /// Include elements (comma-separated).
    pub elements_inc: Option<String>,
    /// Exclude elements (comma-separated).
    pub elements_exc: Option<String>,
    /// Crystal system filter.
    pub crystal_system: Option<Vec<CrystalSystem>>,
    /// Cleavage type filter.
    pub cleavagetype: Option<Vec<CleavageType>>,
    /// Fracture type filter.
    pub fracturetype: Option<Vec<FractureType>>,
    /// Lustre type filter.
    pub lustretype: Option<Vec<LustreType>>,
    /// Diapheny (transparency) filter.
    pub diapheny: Option<Vec<Diapheny>>,
    /// Tenacity filter.
    pub tenacity: Option<Vec<Tenacity>>,
    /// Colour filter.
    pub colour: Option<String>,
    /// Streak filter.
    pub streak: Option<String>,
    /// Optical type filter.
    pub opticaltype: Option<OpticalType>,
    /// Optical sign filter.
    pub opticalsign: Option<OpticalSign>,
    /// Hardness minimum (Mohs).
    pub hardness_min: Option<f32>,
    /// Hardness maximum (Mohs).
    pub hardness_max: Option<f32>,
    /// Density minimum.
    pub density_min: Option<f64>,
    /// Density maximum.
    pub density_max: Option<f64>,
    /// Refractive index minimum.
    pub ri_min: Option<f32>,
    /// Refractive index maximum.
    pub ri_max: Option<f32>,
    /// Birefringence minimum.
    pub bi_min: Option<String>,
    /// Birefringence maximum.
    pub bi_max: Option<String>,
    /// 2V minimum.
    pub optical2v_min: Option<String>,
    /// 2V maximum.
    pub optical2v_max: Option<String>,
    /// Variety of (geomaterial ID).
    pub varietyof: Option<i32>,
    /// Synonym of (geomaterial ID).
    pub synid: Option<i32>,
    /// Polytype of (geomaterial ID).
    pub polytypeof: Option<i32>,
    /// Group ID.
    pub groupid: Option<i32>,
    /// Filter by IDs.
    pub id_in: Option<Vec<i32>>,
    /// Include non-UTF names.
    pub non_utf: Option<bool>,
    /// Meteoritical code filter.
    pub meteoritical_code: Option<String>,
    /// Meteoritical code exists.
    pub meteoritical_code_exists: Option<bool>,
    /// Updated after datetime.
    pub updated_at: Option<String>,
    /// Fields to include.
    pub fields: Option<String>,
    /// Fields to omit.
    pub omit: Option<String>,
    /// Fields to expand.
    pub expand: Option<Vec<String>>,
    /// Ordering.
    pub ordering: Option<GeomaterialsOrdering>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl GeomaterialsQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by name (supports * and _ wildcards).
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Search query.
    pub fn search(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    /// Filter to IMA-approved minerals only.
    pub fn ima_approved(mut self, approved: bool) -> Self {
        self.ima = Some(approved);
        self
    }

    /// Filter by entry type.
    pub fn entry_types(mut self, types: Vec<u8>) -> Self {
        self.entrytype = Some(types);
        self
    }

    /// Filter by included elements.
    pub fn with_elements(mut self, elements: impl Into<String>) -> Self {
        self.elements_inc = Some(elements.into());
        self
    }

    /// Filter by excluded elements.
    pub fn without_elements(mut self, elements: impl Into<String>) -> Self {
        self.elements_exc = Some(elements.into());
        self
    }

    /// Filter by crystal system.
    pub fn crystal_systems(mut self, systems: Vec<CrystalSystem>) -> Self {
        self.crystal_system = Some(systems);
        self
    }

    /// Filter by hardness range.
    pub fn hardness_range(mut self, min: f32, max: f32) -> Self {
        self.hardness_min = Some(min);
        self.hardness_max = Some(max);
        self
    }

    /// Filter by density range.
    pub fn density_range(mut self, min: f64, max: f64) -> Self {
        self.density_min = Some(min);
        self.density_max = Some(max);
        self
    }

    /// Select specific fields.
    pub fn select_fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    /// Omit specific fields.
    pub fn omit_fields(mut self, fields: impl Into<String>) -> Self {
        self.omit = Some(fields.into());
        self
    }

    /// Expand related fields.
    pub fn expand_fields(mut self, fields: Vec<String>) -> Self {
        self.expand = Some(fields);
        self
    }

    /// Set ordering.
    pub fn order_by(mut self, ordering: GeomaterialsOrdering) -> Self {
        self.ordering = Some(ordering);
        self
    }

    /// Set page number.
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }
}
