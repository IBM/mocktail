use crate::body::Body;

/// The body of a mock request or response as a form.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct FormBody {
    fields: Vec<(String, String)>,
}

impl FormBody {
    /// Creates a new form.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Creates an empty form.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Adds a field to the form.
    pub fn field(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let mut fields = self.fields;

        fields.push((name.into(), value.into()));

        Self { fields }
    }

    /// Converts the form body to a URL-encoded string.
    pub fn url_encoded(&self) -> String {
        self.fields
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&")
    }
}

impl PartialEq<Body> for FormBody {
    fn eq(&self, other: &Body) -> bool {
        let mut body_copy = other.clone();
        let other_form_body =
            serde_urlencoded::from_bytes::<Vec<(String, String)>>(&body_copy.as_bytes());

        match other_form_body {
            Ok(other) => self.fields == other,
            Err(_) => false,
        }
    }
}
