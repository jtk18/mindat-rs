//! Reference (bibliography) types for the Mindat API.
//!
//! These endpoints expose Mindat's literature/reference database: the
//! references themselves plus supporting lookup tables (authors, citations,
//! types, languages, ISBN/DDC/LCC classifications, etc.).

use serde::{Deserialize, Serialize};

use super::serde_helpers::{deserialize_i64, deserialize_optional_i32, deserialize_optional_i64};

/// A bibliographic reference (`/references/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Reference ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub ref_id: i64,
    /// Historical ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub ref_his_id: Option<i64>,
    /// Entry date.
    #[serde(default)]
    pub ref_entrydate: Option<String>,
    /// Edit date.
    #[serde(default)]
    pub ref_editdate: Option<String>,
    /// Raw (unformatted) reference text.
    #[serde(default)]
    pub ref_rawtext: Option<String>,
    /// HTML-formatted reference text.
    #[serde(default)]
    pub ref_htmltext: Option<String>,
    /// Reference type ID (see [`ReferenceType`]).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_type: Option<i32>,
    /// Access level.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_access: Option<i32>,
    /// Title.
    #[serde(default)]
    pub ref_title: Option<String>,
    /// Journal name.
    #[serde(default)]
    pub ref_journal: Option<String>,
    /// Publication year.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_year: Option<i32>,
    /// End year (for a range).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_year_to: Option<i32>,
    /// Month.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_month: Option<i32>,
    /// Day.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_day: Option<i32>,
    /// Series number.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_series: Option<i32>,
    /// Volume (as text).
    #[serde(default)]
    pub ref_volume: Option<String>,
    /// Volume (as integer).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_volume_int: Option<i32>,
    /// Issue (as text).
    #[serde(default)]
    pub ref_issue: Option<String>,
    /// Issue (as integer).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_issue_int: Option<i32>,
    /// First page (as integer).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_pagefrom: Option<i32>,
    /// Last page (as integer).
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_pageto: Option<i32>,
    /// First page (as text).
    #[serde(default)]
    pub ref_pagefrom_txt: Option<String>,
    /// Last page (as text).
    #[serde(default)]
    pub ref_pageto_txt: Option<String>,
    /// Publisher.
    #[serde(default)]
    pub ref_publisher: Option<String>,
    /// Place of publication.
    #[serde(default)]
    pub ref_place: Option<String>,
    /// URL.
    #[serde(default)]
    pub ref_url: Option<String>,
    /// File URL.
    #[serde(default)]
    pub ref_file_url: Option<String>,
    /// DOI.
    #[serde(default)]
    pub ref_doi: Option<String>,
    /// Other fields (free text).
    #[serde(default)]
    pub ref_other_fields: Option<String>,
    /// Notes.
    #[serde(default)]
    pub ref_notes: Option<String>,
    /// Deleted-by user ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_deleted_by: Option<i32>,
    /// Issue title.
    #[serde(default)]
    pub ref_issuetitle: Option<String>,
    /// Edition.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_edition: Option<i32>,
    /// Parent reference ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub ref_parent: Option<i64>,
    /// Part.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_part: Option<i32>,
    /// Replaced-by reference ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub ref_replaced_by: Option<i64>,
    /// ISBN.
    #[serde(default)]
    pub ref_isbn: Option<String>,
    /// Abstract.
    #[serde(default)]
    pub ref_abstract: Option<String>,
    /// Last automatic download date.
    #[serde(default)]
    pub ref_lastautodl: Option<String>,
    /// Top-level flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ref_toplevel: Option<i32>,
    /// Journal (transliterated).
    #[serde(default)]
    pub ref_journal_t: Option<String>,
    /// Title (transliterated).
    #[serde(default)]
    pub ref_title_t: Option<String>,
}

/// An author linked to a reference (`/reference-authors/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceAuthor {
    /// Author-link ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub ra_id: i64,
    /// Reference ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub ra_ref_id: Option<i64>,
    /// Author name.
    #[serde(default)]
    pub ra_name: Option<String>,
    /// Author type (e.g. author vs. editor).
    #[serde(default)]
    pub ra_type: Option<serde_json::Value>,
}

/// A de-duplicated author name (`/reference-authors-unique/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceAuthorUnique {
    /// Unique-author ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub rau_id: i64,
    /// Author name.
    #[serde(default)]
    pub rau_text: Option<String>,
}

/// A citation linking a reference to a mineral/locality/photo (`/reference-citations/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceCitation {
    /// Citation ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub rc_id: i64,
    /// Historical ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_his_id: Option<i64>,
    /// Reference ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_ref_id: Option<i64>,
    /// Reference-part ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_rp_id: Option<i64>,
    /// Citation date.
    #[serde(default)]
    pub rc_date: Option<String>,
    /// Creation date.
    #[serde(default)]
    pub rc_date_created: Option<String>,
    /// Mineral ID cited.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_min: Option<i64>,
    /// Locality ID cited.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_loc: Option<i64>,
    /// Article ID cited.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_article: Option<i64>,
    /// Photo ID cited.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_photo: Option<i64>,
    /// Glossary ID cited.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_glossary: Option<i64>,
    /// Entry ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_entryid: Option<i64>,
    /// First page.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rc_pagefrom: Option<i32>,
    /// Last page.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rc_pageto: Option<i32>,
    /// Citation number.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rc_citationno: Option<i32>,
    /// Comments.
    #[serde(default)]
    pub rc_comments: Option<String>,
    /// Deleted-by user ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rc_deletedby: Option<i32>,
    /// Citation type.
    #[serde(default)]
    pub rc_type: Option<serde_json::Value>,
    /// Unique key.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_ukey: Option<i64>,
}

/// A reference type (`/reference-types/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceType {
    /// Type ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub rt_id: i64,
    /// Type code.
    #[serde(default)]
    pub rt_code: Option<String>,
    /// Type name.
    #[serde(default)]
    pub rt_name: Option<String>,
    /// Important flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rt_important: Option<i32>,
}

/// A reference language (`/reference-languages/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceLanguage {
    /// Language-link ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub rl_id: i64,
    /// Reference ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rl_ref_id: Option<i64>,
    /// ISO language code.
    #[serde(default)]
    pub rl_iso: Option<String>,
    /// Minor-language flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub rl_minor: Option<i32>,
}

/// A reference classification entry (`/reference-classify/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceClassify {
    /// Classification ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub rc_id: i64,
    /// Reference ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub rc_ref_id: Option<i64>,
    /// Classification type.
    #[serde(default)]
    pub rc_type: Option<serde_json::Value>,
    /// Classification value.
    #[serde(default)]
    pub rc_value: Option<String>,
}

/// A Dewey Decimal Classification entry (`/reference-ddc/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceDdc {
    /// DDC code.
    #[serde(default)]
    pub ddc_code: String,
    /// DDC title.
    #[serde(default)]
    pub ddc_title: Option<String>,
}

/// A Library of Congress Classification entry (`/reference-lcc/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceLcc {
    /// LCC code.
    #[serde(default)]
    pub lcc_code: String,
    /// LCC title.
    #[serde(default)]
    pub lcc_title: Option<String>,
}

/// An ISBN entry (`/reference-isbn/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceIsbn {
    /// ISBN.
    #[serde(default)]
    pub ri_isbn: String,
    /// Series.
    #[serde(default)]
    pub ri_series: Option<String>,
    /// Publisher.
    #[serde(default)]
    pub ri_publisher: Option<String>,
    /// Year.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ri_year: Option<i32>,
    /// Title.
    #[serde(default)]
    pub ri_title: Option<String>,
}

/// A supplementary reference-extra entry (`/reference-extra/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceExtra {
    /// Extra ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub re_id: i64,
    /// Code.
    #[serde(default)]
    pub re_code: Option<String>,
    /// Text.
    #[serde(default)]
    pub re_text: Option<String>,
}

/// Builder for `/references/` query parameters.
#[derive(Debug, Clone, Default)]
pub struct ReferencesQuery {
    /// Free-text search.
    pub q: Option<String>,
    /// Filter by reference type ID.
    pub ref_type: Option<i32>,
    /// Filter by title.
    pub ref_title: Option<String>,
    /// Filter by journal.
    pub ref_journal: Option<String>,
    /// Filter by publisher.
    pub ref_publisher: Option<String>,
    /// Filter by DOI.
    pub ref_doi: Option<String>,
    /// Filter by ISBN.
    pub ref_isbn: Option<String>,
    /// Filter by a set of IDs.
    pub id_in: Option<Vec<i64>>,
    /// Minimum ID (range).
    pub id_range_min: Option<i64>,
    /// Maximum ID (range).
    pub id_range_max: Option<i64>,
    /// Ordering field.
    pub ordering: Option<String>,
    /// Fields to include.
    pub fields: Option<String>,
    /// Fields to omit.
    pub omit: Option<String>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl ReferencesQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Free-text search across references.
    pub fn search(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    /// Filter by title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.ref_title = Some(title.into());
        self
    }

    /// Filter by journal.
    pub fn journal(mut self, journal: impl Into<String>) -> Self {
        self.ref_journal = Some(journal.into());
        self
    }

    /// Filter by DOI.
    pub fn doi(mut self, doi: impl Into<String>) -> Self {
        self.ref_doi = Some(doi.into());
        self
    }

    /// Filter by reference type ID.
    pub fn ref_type(mut self, ref_type: i32) -> Self {
        self.ref_type = Some(ref_type);
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

/// Builder for `/reference-citations/` query parameters.
#[derive(Debug, Clone, Default)]
pub struct ReferenceCitationsQuery {
    /// Filter by cited mineral ID.
    pub rc_min: Option<i64>,
    /// Filter by cited locality ID.
    pub rc_loc: Option<i64>,
    /// Filter by cited photo ID.
    pub rc_photo: Option<i64>,
    /// Filter by cited article ID.
    pub rc_article: Option<i64>,
    /// Filter by reference ID.
    pub rc_ref_id: Option<i64>,
    /// Filter by entry ID.
    pub rc_entryid: Option<i64>,
    /// Filter by citation type.
    pub rc_type: Option<i32>,
    /// Ordering field.
    pub ordering: Option<String>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl ReferenceCitationsQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter to citations of a given mineral.
    pub fn mineral(mut self, mineral_id: i64) -> Self {
        self.rc_min = Some(mineral_id);
        self
    }

    /// Filter to citations of a given locality.
    pub fn locality(mut self, locality_id: i64) -> Self {
        self.rc_loc = Some(locality_id);
        self
    }

    /// Filter to citations of a given reference.
    pub fn reference(mut self, ref_id: i64) -> Self {
        self.rc_ref_id = Some(ref_id);
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
