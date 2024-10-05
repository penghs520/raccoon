mod card_types;
mod customize_fields;
mod relationships;
mod biz_rules;
mod schema;
mod work_flows;

#[cfg(test)]
mod tests {
    use common::id_generator::IdGenerator;

    #[test]
    fn it_works() {
        let id  = String::generate_id();
        println!("{}", id);
    }
}