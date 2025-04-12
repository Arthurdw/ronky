use crate::PropertiesSchema;

pub trait Exportable {
    fn export() -> PropertiesSchema;
}
