use ksuid::Ksuid;
use rocket::http::RawStr;
use rocket::request::FromParam;

pub struct KsuidWrapper {
    wrapped: Ksuid,
}

impl KsuidWrapper {
    pub fn unwrap(self) -> Ksuid {
        self.wrapped
    }
}

impl<'r> FromParam<'r> for KsuidWrapper {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let ksuid_string: String = param.parse().map_err(|_| param)?;
        let wrapped = Ksuid::from_base62(&ksuid_string).map_err(|_| param)?;
        Ok(KsuidWrapper { wrapped })
    }
}
