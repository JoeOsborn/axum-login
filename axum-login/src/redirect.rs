use axum::http::{self, Uri};

/// This is intended for internal use only and subject to change in the future
/// without warning!
#[doc(hidden)]
pub fn url_with_redirect_query(
    url: &str,
    redirect_field: &str,
    redirect_uri: Uri,
) -> Result<Uri, http::Error> {
    let uri = url.parse::<Uri>()?;
    let target = String::new();
    let mut serializer = form_urlencoded::Serializer::new(target);
    if let Some(query) = uri.query() {
        let parsed = form_urlencoded::parse(query.as_bytes());
        let has_redirect = parsed.clone().any(|(key, _)| key == redirect_field);
        if has_redirect {
            return Ok(uri);
        }
        serializer.extend_pairs(parsed);
    }
    serializer.append_pair(redirect_field, &redirect_uri.to_string());
    let query = serializer.finish();
    let mut parts = uri.clone().into_parts();
    parts.path_and_query = Some(format!("{}?{}",uri.path(),query).parse()?);
    Ok(Uri::from_parts(parts)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_redirect_when_query_contains_substring() {
        let redirect = "/dashboard".parse::<Uri>().unwrap();
        let uri = url_with_redirect_query("/login?context=1", "next", redirect).unwrap();

        assert_eq!(uri.to_string(), "/login?context=1&next=%2Fdashboard");
    }

    #[test]
    fn preserves_existing_redirect_param() {
        let redirect = "/dashboard".parse::<Uri>().unwrap();
        let uri =
            url_with_redirect_query("/login?next=%2Fkeep&context=1", "next", redirect).unwrap();

        assert_eq!(uri.to_string(), "/login?next=%2Fkeep&context=1");
    }
}
