use chrono;
use uuid::Uuid;
use juniper::Value;

pub struct ID(pub Uuid);
impl Clone for ID {
    fn clone(&self) -> ID {
        ID(self.0.clone())
    }
}

pub struct DateTime(pub chrono::DateTime<chrono::Utc>);
impl Clone for DateTime {
    fn clone(&self) -> DateTime {
        DateTime(self.0.clone())
    }
}

graphql_scalar!(ID {
    description: "converts uuid's to strings and back again"

    resolve(&self) -> Value {
        Value::String(self.0.hyphenated().to_string())
    }

    from_input_value(v: &InputValue) -> Option<ID> {
        match v.as_string_value() {
            Some(string_value) => {
                match Uuid::parse_str(string_value) {
                    Ok(uuid_) => {
                        Some(ID(uuid_))
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }
});

graphql_scalar!(DateTime {
    description: "datetimes to iso8601 strings"

    resolve(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }

    from_input_value(v: &InputValue) -> Option<DateTime> {
        match v.as_string_value() {
            Some(string_value) => {
                match chrono::DateTime::parse_from_rfc3339(string_value) {
                    Ok(datetime) => {
                        Some(DateTime(datetime.with_timezone(&chrono::Utc)))
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }
});
