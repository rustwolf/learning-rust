// use struct_mod;

#[cfg(test)]
pub mod test_structs {

    use mockall_double::double;

    use crate::struct_mod::get_activity;

    // #[double]
    // use crate::struct_mod::get_activity;
    // mod struct_mod;

    #[test]
    pub fn test_me() {
        assert_eq!(1, 1);

        let data = get_activity();

        assert_eq!(data.citizen, true);
    }
}
