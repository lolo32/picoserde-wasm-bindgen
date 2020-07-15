macro_rules! l {
    ($target:ident, $line:expr) => {
        $target.push_str($line);
    };

    ($target:ident, $line:expr, $($param:expr),*) => {
        $target.push_str(&format!($line, $($param,)*));
    };
}

pub fn attrs_proxy(attributes: &[crate::parse::Attribute]) -> Option<String> {
    attributes.iter().find_map(|attr| {
        if attr.tokens.len() == 2 && attr.tokens[0] == "proxy".to_string() {
            Some(attr.tokens[1].clone())
        } else {
            None
        }
    })
}

pub fn attrs_rename(attributes: &[crate::parse::Attribute]) -> Option<String> {
    attributes.iter().find_map(|attr| {
        if attr.tokens.len() == 2 && attr.tokens[0] == "rename".to_string() {
            Some(attr.tokens[1].clone())
        } else {
            None
        }
    })
}

pub fn attrs_default(attributes: &[crate::parse::Attribute]) -> bool {
    attributes
        .iter()
        .any(|attr| attr.tokens.len() == 1 && attr.tokens[0] == "default".to_string())
}