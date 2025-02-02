use rome_formatter_test::check_reformat::CheckReformat;
use rome_js_formatter::context::{JsFormatOptions, QuoteStyle, Semicolons};
use rome_js_formatter::format_node;
use rome_js_parser::{parse, JsParserOptions};
use rome_js_syntax::JsFileSource;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
class Foo {
				constructor(
				@dec
				/*leading parameter*/ private parameter
				) {	}
			}


"#;
    let syntax = JsFileSource::tsx();
    let tree = parse(
        src,
        syntax,
        JsParserOptions::default().with_parse_class_parameter_decorators(),
    );
    dbg!(tree.syntax());
    let options = JsFormatOptions::new(syntax)
        .with_semicolons(Semicolons::AsNeeded)
        .with_quote_style(QuoteStyle::Double)
        .with_jsx_quote_style(QuoteStyle::Single);
    let result = format_node(options.clone(), &tree.syntax())
        .unwrap()
        .print()
        .unwrap();

    let root = &tree.syntax();
    let language = language::JsTestFormatLanguage::new(JsFileSource::tsx());
    let check_reformat =
        CheckReformat::new(root, result.as_code(), "quick_test", &language, options);
    check_reformat.check_reformat();

    assert_eq!(
        result.as_code(),
        r#"
        // A
@Foo()
// B
@Bar()
// C
export class Bar{}

"#
    );
}
