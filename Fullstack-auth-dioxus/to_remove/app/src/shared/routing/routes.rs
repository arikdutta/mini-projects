use crate::utils::param::PARAM;
use crate::utils::types::identifiable::Identifiable;

pub trait Routes {
    type Pk: Sized + 'static;

    fn base_path() -> &'static str;

    fn list_path() -> &'static str {
        Self::base_path()
    }

    fn new_path() -> &'static str {
        "/"
    }

    fn read_path() -> &'static str {
        Self::base_path()
    }

    fn read_param() -> &'static str {
        PARAM::ID
    }

    fn read_url<E>(entity: &E) -> String
    where
        E: Identifiable<Pk = Self::Pk>,
    {
        let path = Self::read_path()
            .trim_start_matches('/')
            .trim_end_matches('/');
        format!("/{}/{}", path, entity.id())
    }

    fn edit_path() -> &'static str {
        Self::read_path()
    }

    fn edit_param() -> &'static str {
        Self::read_param()
    }

    fn edit_url<E>(entity: &E) -> String
    where
        E: Identifiable<Pk = Self::Pk>,
    {
        let path = Self::edit_path()
            .trim_start_matches('/')
            .trim_end_matches('/');
        format!("/{}/{}", path, entity.id())
    }
}
