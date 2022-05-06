use rome_formatter::FormatResult;
use rome_js_syntax::JsAnyStatement;
use rome_rowan::AstNodeList;

use crate::{
    format_elements, hard_line_break, indent, space_token, Format, FormatElement, FormatNode,
    Formatter, JsFormatter,
};

use rome_js_syntax::JsCaseClause;
use rome_js_syntax::JsCaseClauseFields;

impl FormatNode for JsCaseClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCaseClauseFields {
            case_token,
            test,
            colon_token,
            consequent,
        } = self.as_fields();

        let is_first_child_block_stmt = matches!(
            consequent.iter().next(),
            Some(JsAnyStatement::JsBlockStatement(_))
        );
        let case_word = case_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let test = test.format(formatter)?;
        let cons = formatter.format_list(consequent);

        let cons = if cons.is_empty() {
            // Skip inserting an indent block is the consequent is empty to print
            // the trailing comments for the case clause inline if there is no
            // block to push them into
            hard_line_break()
        } else if is_first_child_block_stmt {
            format_elements![space_token(), cons]
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), cons])
        };

        Ok(format_elements![
            case_word,
            space_token(),
            test,
            colon,
            cons
        ])
    }
}