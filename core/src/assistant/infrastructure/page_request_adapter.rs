use crate::assistant::domain::dto::PageRequest;

#[derive(Default)]
pub struct DbPageRequest {
    pub after: Option<i32>,
    pub before: Option<i32>,
    pub limit: Option<i32>,
}

impl From<&PageRequest> for DbPageRequest {
    fn from(value: &PageRequest) -> Self {
        let after: Option<i32> = match &value.after {
            Some(after) => Some(after.parse().unwrap()),
            None => None,
        };
        let before: Option<i32> = match &value.before {
            Some(before) => Some(before.parse().unwrap()),
            None => None,
        };
        let limit: Option<i32> = match &value.limit {
            Some(limit) => Some(limit.parse().unwrap()),
            None => None,
        };

        DbPageRequest {
            after,
            before,
            limit,
        }
    }
}
