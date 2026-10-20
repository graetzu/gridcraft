use gridcraft_formula::{FormulaLocale, parse, parse_input, print, print_input};

fn input(src: &str) -> String {
    print(&parse_input(src, FormulaLocale::De, |_| false).unwrap())
}

#[test]
fn german_aliases_and_argument_separators() {
    for (src, expected) in [
        ("=SUMME(A1:A10)", "SUM(A1:A10)"),
        ("MITTELWERT(A1:A10)", "AVERAGE(A1:A10)"),
        ("WENN(A1>0;\"Ja\";\"Nein\")", "IF(A1>0,\"Ja\",\"Nein\")"),
        ("ANZAHL(A1:A10)", "COUNT(A1:A10)"),
        ("SUMMEWENN(A1:A10;\">0\")", "SUMIF(A1:A10,\">0\")"),
        ("ZÄHLENWENN(A1:A10;\">0\")", "COUNTIF(A1:A10,\">0\")"),
        ("SVERWEIS(A2;D:E;2;FALSCH)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("HEUTE()", "TODAY()"),
        ("=wahr", "TRUE"),
        ("=falsch", "FALSE"),
        ("WAHR()", "TRUE()"),
        ("FALSCH()", "FALSE()"),
        ("SUMME(1;SUM(2;3);MITTELWERT(4;6))", "SUM(1,SUM(2,3),AVERAGE(4,6))"),
        ("WENN(WAHR;;)", "IF(TRUE,,)"),
        ("LAMBDA(x;x+1)(2)", "LAMBDA(x,x+1)(2)"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
}

#[test]
fn german_input_preserves_reference_and_array_syntax() {
    for (src, expected) in [
        ("SUMME({1.2;3.4};5)", "SUM({1,2;3,4},5)"),
        ("{WAHR.FALSCH;TRUE.FALSE}", "{TRUE,FALSE;TRUE,FALSE}"),
        ("SUM((A1:A3;C1:C3))", "SUM((A1:A3,C1:C3))"),
        ("\"SUMME;FALSCH\"&\"a\"\"b\"", "\"SUMME;FALSCH\"&\"a\"\"b\""),
        ("'SUMME;FALSCH'!A1+SUMME!B2+SUMME!FALSCH", "'SUMME;FALSCH'!A1+SUMME!B2+SUMME!FALSCH"),
        ("SUMME[ZÄHLENWENN]+FALSCH[WENN]+ANZAHL", "SUMME[ZÄHLENWENN]+FALSCH[WENN]+ANZAHL"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
    for bad in [r"SUMME({1;2.3})", "{SUMME(A1)}", "{FALSCH;}"] {
        assert!(parse_input(bad, FormulaLocale::De, |_| false).is_err(), "{bad}");
    }
    assert_eq!(print(&parse("SUMME(1,2)+FALSCH").unwrap()), "SUMME(1,2)+FALSCH");
    assert!(parse("SUM(1;2)").is_err());
    assert!(parse("{WAHR}").is_err());
    let long_chain = "FALSCH+".repeat(300) + "WAHR";
    assert!(parse_input(&long_chain, FormulaLocale::De, |_| false).is_err());
    assert!(print_input(&parse(&long_chain).unwrap(), FormulaLocale::De, |_| false).is_err());
}

#[test]
fn german_input_preserves_defined_and_local_names() {
    let known = |name: &str| ["SUMME", "FALSCH"].iter().any(|n| n.eq_ignore_ascii_case(name));
    let expr = parse_input("SUMME(2;3)+FALSCH+WENN(WAHR;4;5)", FormulaLocale::De, known).unwrap();
    assert_eq!(print(&expr), "SUMME(2,3)+FALSCH+IF(TRUE,4,5)");
    for (src, expected) in [
        ("LET(FALSCH;7;FALSCH)", "LET(FALSCH,7,FALSCH)"),
        ("LET(falsch;FALSCH;WENN(falsch;1;2))", "LET(falsch,FALSE,IF(falsch,1,2))"),
        ("LET(SUMME;LAMBDA(x;x+1);SUMME(2))", "LET(SUMME,LAMBDA(x,x+1),SUMME(2))"),
        ("LET(falsch;7;wert;FALSCH;wert)", "LET(falsch,7,wert,FALSCH,wert)"),
        ("LAMBDA(FALSCH;FALSCH)(7)", "LAMBDA(FALSCH,FALSCH)(7)"),
        ("LAMBDA(SUMME;SUMME(2))(LAMBDA(x;x+1))", "LAMBDA(SUMME,SUMME(2))(LAMBDA(x,x+1))"),
        ("LET(FALSCH;7;FALSCH)+FALSCH", "LET(FALSCH,7,FALSCH)+FALSE"),
        ("LET(FALSCH;7;LAMBDA(x;FALSCH+x)(1))", "LET(FALSCH,7,LAMBDA(x,FALSCH+x)(1))"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
}

#[test]
fn locale_controls_decimal_list_array_separators_and_display() {
    let de = FormulaLocale::De;
    let en = FormulaLocale::En;
    let expr = parse_input("WENN(WAHR;SUMME(1,5;2,25);0)", de, |_| false).unwrap();
    assert_eq!(print(&expr), "IF(TRUE,SUM(1.5,2.25),0)");
    assert_eq!(print_input(&expr, de, |_| false).unwrap(), "WENN(WAHR;SUMME(1,5;2,25);0)");
    assert_eq!(print_input(&expr, en, |_| false).unwrap(), print(&expr));
    assert!(parse_input("SUM(1;2)", en, |_| false).is_err());
    assert_eq!(print(&parse_input("SUMME(1,2)+FALSCH", en, |_| false).unwrap()), "SUMME(1,2)+FALSCH");
    assert!(parse_input("SUMME(1.5;2)", de, |_| false).is_err());
    let expr = parse("SUM({1.5,2.25;3,4})+SUM((A1:A3,C1:C3))").unwrap();
    let shown = print_input(&expr, de, |_| false).unwrap();
    assert_eq!(print(&parse_input(&shown, de, |_| false).unwrap()), print(&expr));
    let expr = parse("LET(SUMME,LAMBDA(x,x+1),SUM(SUMME(2),FALSE))").unwrap();
    let shown = print_input(&expr, de, |n| n == "FALSCH").unwrap();
    assert_eq!(shown, "LET(SUMME;LAMBDA(x;x+1);SUM(SUMME(2);FALSE))");
    assert_eq!(print(&parse_input(&shown, de, |n| n == "FALSCH").unwrap()), print(&expr));
}

#[test]
fn german_display_round_trips_text_containing_separators() {
    let de = FormulaLocale::De;
    let expr = parse(r#"IF(A1="a,b;c",CONCAT("1.5",",",";"),SUM(0.25,A1:A3 B2))"#).unwrap();
    let shown = print_input(&expr, de, |_| false).unwrap();
    assert_eq!(shown, r#"WENN(A1="a,b;c";TEXTKETTE("1.5";",";";");SUMME(0,25;A1:A3 B2))"#);
    assert_eq!(print(&parse_input(&shown, de, |_| false).unwrap()), print(&expr));
}
