use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub jml);

#[test]
fn calculator1() {
    assert!(jml::TermParser::new().parse("22").is_ok());
    assert!(jml::TermParser::new().parse("(22)").is_ok());
    assert!(jml::TermParser::new().parse("((((22))))").is_ok());
    assert!(jml::TermParser::new().parse("((22)").is_err());
}
