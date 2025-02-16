use enumflags2::bitflags;
use psl_core::parser_database as db;
use std::fmt;

/// Holds information about a relation field.
#[derive(Debug, PartialEq, Clone)]
pub struct RelationInfo {
    /// The target model of the relation.
    pub referenced_model: String,
    /// The fields forming the relation.
    pub fields: Vec<String>,
    /// The target field of the relation a.k.a. `references`
    pub references: Vec<String>,
    /// The name of the relation. Internally, an empty string signals no name.
    pub name: String,
    /// Foreign Key Constraint Name if there is one
    pub fk_name: Option<String>,
    /// A strategy indicating what happens when
    /// a related node is deleted.
    pub on_delete: Option<ReferentialAction>,
    /// A strategy indicating what happens when
    /// a related node is updated.
    pub on_update: Option<ReferentialAction>,
}

impl RelationInfo {
    /// Creates a new relation info for the
    /// given target model.
    pub fn new(model: &str) -> RelationInfo {
        RelationInfo {
            referenced_model: String::from(model),
            fields: Vec::new(),
            references: Vec::new(),
            name: String::new(),
            fk_name: None,
            on_delete: None,
            on_update: None,
        }
    }
}

/// Describes what happens when related nodes are deleted.
#[repr(u8)]
#[bitflags]
#[derive(Debug, Copy, PartialEq, Clone)]
pub enum ReferentialAction {
    /// Deletes record if dependent record is deleted. Updates relation scalar
    /// fields if referenced scalar fields of the dependent record are updated.
    /// Prevents operation (both updates and deletes) from succeeding if any
    /// records are connected.
    Cascade,
    /// Prevents operation (both updates and deletes) from succeeding if any
    /// records are connected. This behavior will always result in a runtime
    /// error for required relations.
    Restrict,
    /// Behavior is database specific. Either defers throwing an integrity check
    /// error until the end of the transaction or errors immediately. If
    /// deferred, this makes it possible to temporarily violate integrity in a
    /// transaction while making sure that subsequent operations in the
    /// transaction restore integrity.
    /// When using relationMode = "prisma", NoAction becomes an alias of
    /// the emulated Restrict (when supported).
    NoAction,
    /// Sets relation scalar fields to null if the relation is deleted or
    /// updated. This will always result in a runtime error if one or more of the
    /// relation scalar fields are required.
    SetNull,
    /// Sets relation scalar fields to their default values on update or delete
    /// of relation. Will always result in a runtime error if no defaults are
    /// provided for any relation scalar fields.
    SetDefault,
}

impl From<db::ReferentialAction> for ReferentialAction {
    fn from(ra: db::ReferentialAction) -> Self {
        match ra {
            db::ReferentialAction::Cascade => ReferentialAction::Cascade,
            db::ReferentialAction::SetNull => ReferentialAction::SetNull,
            db::ReferentialAction::SetDefault => ReferentialAction::SetDefault,
            db::ReferentialAction::Restrict => ReferentialAction::Restrict,
            db::ReferentialAction::NoAction => ReferentialAction::NoAction,
        }
    }
}

impl AsRef<str> for ReferentialAction {
    fn as_ref(&self) -> &'static str {
        match self {
            ReferentialAction::Cascade => "Cascade",
            ReferentialAction::Restrict => "Restrict",
            ReferentialAction::NoAction => "NoAction",
            ReferentialAction::SetNull => "SetNull",
            ReferentialAction::SetDefault => "SetDefault",
        }
    }
}

impl fmt::Display for ReferentialAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}
