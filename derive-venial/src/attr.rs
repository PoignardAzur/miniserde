use proc_macro2::{Literal, TokenTree};
use venial::{Attribute, EnumVariant, Error, NamedField};

// FIXME - handle attributes with multiple items

/// Find the value of a #[serde(rename = "...")] attribute.
fn attr_rename(attributes: &[Attribute]) -> Result<Option<Literal>, Error> {
    let mut rename = None;

    for attribute in attributes {
        match &attribute.get_single_path_segment() {
            Some(ident) if *ident == "serde" => (),
            _ => continue,
        }

        if rename.is_some() {
            return Err(Error::new_at_tokens(
                &attribute,
                "duplicate rename attribute",
            ));
        }

        let list: Vec<_> = match &attribute.value {
            venial::AttributeValue::Group(_, group) => group.clone(),
            _ => {
                return Err(Error::new_at_tokens(
                    &attribute.value,
                    "unsupported attribute",
                ))
            }
        };

        match list.get(0) {
            Some(TokenTree::Ident(ident)) if ident == "rename" => (),
            Some(TokenTree::Ident(ident)) => {
                return Err(Error::new_at_tokens(&ident, "unsupported attribute"))
            }
            _ => {
                return Err(Error::new_at_tokens(
                    &attribute.value,
                    "unsupported attribute",
                ))
            }
        }
        match list.get(1) {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => (),
            _ => {
                return Err(Error::new_at_tokens(
                    &attribute.value,
                    "unsupported attribute",
                ))
            }
        };
        let literal = match list.get(2) {
            Some(TokenTree::Literal(literal)) => literal,
            _ => {
                return Err(Error::new_at_tokens(
                    &attribute.value,
                    "unsupported attribute",
                ))
            }
        };

        rename = Some(literal.clone());
    }

    Ok(rename)
}

/// Determine the name of a field, respecting a rename attribute.
pub fn name_of_field(field: &NamedField) -> Result<Literal, Error> {
    let rename = attr_rename(&field.attributes)?;
    Ok(rename.unwrap_or_else(|| Literal::string(&field.name.to_string())))
}

/// Determine the name of a variant, respecting a rename attribute.
pub fn name_of_variant(var: &EnumVariant) -> Result<Literal, Error> {
    let rename = attr_rename(&var.attributes)?;
    Ok(rename.unwrap_or_else(|| Literal::string(&var.name.to_string())))
}
