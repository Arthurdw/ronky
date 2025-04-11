use crate::Serializable;

pub trait Exportable {
    fn export() -> impl Serializable;
}
