use indoc::indoc;

use mago_php_version::PHPVersion;
use mago_reporting::*;
use mago_span::*;
use mago_syntax::ast::*;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct RequireConstantTypeRule;

impl Rule for RequireConstantTypeRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Require Constant Type", Level::Warning)
            .with_description(indoc! {"
                Detects class constants that are missing a type hint.
            "})
            .with_minimum_supported_php_version(PHPVersion::PHP83)
            .with_example(RuleUsageExample::valid(
                "A class constant with a type hint",
                indoc! {r#"
                    <?php

                    declare(strict_types=1);

                    namespace Psl\IO\Internal;

                    use Psl\IO;

                    class ResourceHandle implements IO\CloseSeekReadWriteStreamHandleInterface
                    {
                        use IO\ReadHandleConvenienceMethodsTrait;
                        use IO\WriteHandleConvenienceMethodsTrait;

                        public const int DEFAULT_READ_BUFFER_SIZE = 4096;
                        public const int MAXIMUM_READ_BUFFER_SIZE = 786432;

                        // ...
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "A class constant without a type hint",
                indoc! {r#"
                    <?php

                    declare(strict_types=1);

                    namespace Psl\IO\Internal;

                    use Psl\IO;

                    class ResourceHandle implements IO\CloseSeekReadWriteStreamHandleInterface
                    {
                        use IO\ReadHandleConvenienceMethodsTrait;
                        use IO\WriteHandleConvenienceMethodsTrait;

                        public const DEFAULT_READ_BUFFER_SIZE = 4096;
                        public const MAXIMUM_READ_BUFFER_SIZE = 786432;

                        // ...
                    }
                "#},
            ))
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::ClassLikeConstant(class_like_constant) = node else { return LintDirective::default() };

        if class_like_constant.hint.is_some() {
            return LintDirective::Prune;
        }

        let item = class_like_constant.first_item();

        let constant_name = context.lookup(&item.name.value);

        context.report(
            Issue::new(context.level(), format!("Class constant `{constant_name}` is missing a type hint."))
                .with_annotation(
                    Annotation::primary(class_like_constant.span())
                        .with_message(format!("Class constant `{constant_name}` is defined here.")),
                )
                .with_note("Adding a type hint to constants improves code readability and helps prevent type errors.")
                .with_help(format!("Consider specifying a type hint for `{constant_name}`.")),
        );

        LintDirective::Prune
    }
}
