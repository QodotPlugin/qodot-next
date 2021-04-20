use gdnative::{Dictionary, Variant};

pub fn build_property_dictionary(
    property_name: &str,
    property_type: i64,
    property_hint: Option<i64>,
    hint_string: Option<&str>,
    usage: Option<i64>,
) -> Dictionary {
    let mut property_dictionary = Dictionary::new();
    property_dictionary.set(
        &Variant::from_str("name"),
        &Variant::from_str(property_name),
    );
    property_dictionary.set(
        &Variant::from_str("type"),
        &Variant::from_i64(property_type),
    );

    if let Some(property_hint) = property_hint {
        property_dictionary.set(
            &Variant::from_str("hint"),
            &Variant::from_i64(property_hint),
        );
    }

    if let Some(hint_string) = hint_string {
        property_dictionary.set(
            &Variant::from_str("hint_string"),
            &Variant::from_str(hint_string),
        );
    }

    if let Some(usage) = usage {
        property_dictionary.set(&Variant::from_str("usage"), &Variant::from_i64(usage));
    }

    property_dictionary
}
