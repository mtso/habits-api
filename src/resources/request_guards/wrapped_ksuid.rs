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

///// Allows the use of `User` as a Rocket request guard.
//impl<'r> FromParam<'r> for Wrapper<Ksuid> {
//    type Error = &'r RawStr;
//
//    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
//        let (key, val_str) = match param.find(':') {
//            Some(i) if i > 0 => (&param[..i], &param[(i + 1)..]),
//            _ => return Err(param)
//        };
//
//        if !key.chars().all(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
//            return Err(param);
//        }
//
//        val_str.parse().map(|value| {
//            MyParam {
//                key: key,
//                value: value
//            }
//        }).map_err(|_| param)
//    }
//}
