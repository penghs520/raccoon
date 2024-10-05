mod card_types;
mod customize_fields;
mod relationships;
mod biz_rules;
mod schema;
mod work_flows;

#[cfg(test)]
mod tests {
    use common::id_generator::IdGenerator;
    use crate::card_types::{CardType, CommonTraitType};

    #[test]
    fn it_works() {
        let id = String::generate_id();
        println!("{}", id);
    }

    #[test]
    fn test_card_type_definition_serde() {
        let common_trait_type = CardType::CommonTraitType(CommonTraitType::new(
            String::generate_id(),
            String::generate_id(),
            String::generate_id(),
            Some(String::generate_id()),
        ));
        //println!("{:?}", common_trait_type);
        let json = serde_json::to_string(&common_trait_type).unwrap();
        println!("{}", json);
    }
}