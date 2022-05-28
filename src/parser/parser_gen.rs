use super::MiniImp;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    number,
    string_literal,
    identifier,
    WHITESPACE,
    COMMENT,
    plus,
    minus,
    multi,
    divide,
    true_bool,
    false_bool,
    boolean,
    is,
    not,
    truth,
    factor,
    mul,
    expr,
    arg,
    if_stmt,
    while_stmt,
    set_stmt,
    var_stmt,
    program,
    call_stmt,
    stmt,
    stmts,
    scope,
    file,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for MiniImp {
    fn parse<'i>(
        rule: Rule,
        input: &'i str,
    ) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.sequence(|state| {
                            state
                                .repeat(|state| super::visible::WHITESPACE(state))
                                .and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            super::visible::COMMENT(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    super::visible::WHITESPACE(state)
                                                })
                                            })
                                        })
                                    })
                                })
                        })
                    } else {
                        Ok(state)
                    }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn number(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::number, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::ASCII_DIGIT(state).and_then(|state| {
                                    state.repeat(|state| self::ASCII_DIGIT(state))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_literal(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::string_literal, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("\"")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::ASCII_ALPHANUMERIC(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::ASCII_ALPHANUMERIC(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("\""))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn identifier(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::identifier, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::ASCII_ALPHANUMERIC(state).and_then(|state| {
                                    state.repeat(|state| self::ASCII_ALPHANUMERIC(state))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| {
                        state
                            .match_string(" ")
                            .or_else(|state| self::NEWLINE(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| {
                        state
                            .sequence(|state| {
                                state
                                    .match_string("/*")
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| {
                                                        state.match_string("*/")
                                                    })
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| state.match_string("*/"))
                            })
                            .or_else(|state| {
                                state.sequence(|state| {
                                    state
                                        .match_string("//")
                                        .and_then(|state| state.repeat(|state| self::ANY(state)))
                                })
                            })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn plus(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::plus, |state| state.match_string("+"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn minus(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::minus, |state| state.match_string("-"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn multi(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::multi, |state| state.match_string("*"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn divide(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::divide, |state| state.match_string("/"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn true_bool(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::true_bool, |state| state.match_string("true"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn false_bool(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::false_bool, |state| state.match_string("false"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn boolean(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::boolean, |state| {
                        self::true_bool(state).or_else(|state| self::false_bool(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn is(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::is, |state| state.match_string("is"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn not(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::not, |state| state.match_string("not"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn truth(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::truth, |state| {
                        self::true_bool(state)
                            .or_else(|state| self::false_bool(state))
                            .or_else(|state| {
                                state.sequence(|state| {
                                    self::not(state)
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| self::truth(state))
                                })
                            })
                            .or_else(|state| {
                                state.sequence(|state| {
                                    self::is(state)
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| self::identifier(state))
                                        .and_then(|state| super::hidden::skip(state))
                                        .and_then(|state| self::expr(state))
                                })
                            })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn factor(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::factor, |state| {
                        state
                            .sequence(|state| {
                                state
                                    .match_string("(")
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| self::expr(state))
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| state.match_string(")"))
                            })
                            .or_else(|state| self::number(state))
                            .or_else(|state| self::identifier(state))
                            .or_else(|state| self::string_literal(state))
                            .or_else(|state| self::boolean(state))
                            .or_else(|state| self::truth(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn mul(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::mul, |state| {
                        state.sequence(|state| {
                            self::factor(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::multi(state)
                                                        .or_else(|state| self::divide(state))
                                                        .and_then(|state| {
                                                            super::hidden::skip(state)
                                                        })
                                                        .and_then(|state| self::factor(state))
                                                })
                                                .and_then(|state| {
                                                    state.repeat(|state| {
                                                        state.sequence(|state| {
                                                            super::hidden::skip(state).and_then(
                                                                |state| {
                                                                    state.sequence(|state| {
                                                                        self::multi(state)
                                                                            .or_else(|state| {
                                                                                self::divide(state)
                                                                            })
                                                                            .and_then(|state| {
                                                                                super::hidden::skip(
                                                                                    state,
                                                                                )
                                                                            })
                                                                            .and_then(|state| {
                                                                                self::factor(state)
                                                                            })
                                                                    })
                                                                },
                                                            )
                                                        })
                                                    })
                                                })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expr, |state| {
                        state.sequence(|state| {
                            self::mul(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::plus(state)
                                                        .or_else(|state| self::minus(state))
                                                        .and_then(|state| {
                                                            super::hidden::skip(state)
                                                        })
                                                        .and_then(|state| self::mul(state))
                                                })
                                                .and_then(|state| {
                                                    state.repeat(|state| {
                                                        state.sequence(|state| {
                                                            super::hidden::skip(state).and_then(
                                                                |state| {
                                                                    state.sequence(|state| {
                                                                        self::plus(state)
                                                                            .or_else(|state| {
                                                                                self::minus(state)
                                                                            })
                                                                            .and_then(|state| {
                                                                                super::hidden::skip(
                                                                                    state,
                                                                                )
                                                                            })
                                                                            .and_then(|state| {
                                                                                self::mul(state)
                                                                            })
                                                                    })
                                                                },
                                                            )
                                                        })
                                                    })
                                                })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn arg(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::arg, |state| {
                        state.sequence(|state| {
                            self::expr(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.match_string(",").or_else(|state| {
                                        state.lookahead(true, |state| state.match_string(")"))
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_stmt(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::if_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("if")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("then"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("end."))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn while_stmt(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::while_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("while")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::scope(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn set_stmt(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::set_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("set")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("="))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(";"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn var_stmt(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::var_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("var")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.optional(|state| {
                                        state.sequence(|state| {
                                            state
                                                .match_string("=")
                                                .and_then(|state| super::hidden::skip(state))
                                                .and_then(|state| self::expr(state))
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(";"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::program, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("program")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::scope(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn call_stmt(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::call_stmt, |state| {
                        state.sequence(|state| {
                            self::expr(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("("))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::arg(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state)
                                                            .and_then(|state| self::arg(state))
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(")"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(";"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn stmt(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::stmt, |state| {
                        self::if_stmt(state)
                            .or_else(|state| self::while_stmt(state))
                            .or_else(|state| self::set_stmt(state))
                            .or_else(|state| self::var_stmt(state))
                            .or_else(|state| self::program(state))
                            .or_else(|state| self::call_stmt(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn stmts(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::stmts, |state| {
                        state.sequence(|state| {
                            state.optional(|state| {
                                self::stmt(state).and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            super::hidden::skip(state)
                                                .and_then(|state| self::stmt(state))
                                        })
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn scope(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::scope, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("begin")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("end."))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn file(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::file, |state| {
                        state.sequence(|state| {
                            self::SOI(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::EOI(state))
                        })
                    })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHANUMERIC(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_range('a'..'z')
                        .or_else(|state| state.match_range('A'..'Z'))
                        .or_else(|state| state.match_range('0'..'9'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_string("\n")
                        .or_else(|state| state.match_string("\r\n"))
                        .or_else(|state| state.match_string("\r"))
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::number => rules::number(state),
            Rule::string_literal => rules::string_literal(state),
            Rule::identifier => rules::identifier(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::plus => rules::plus(state),
            Rule::minus => rules::minus(state),
            Rule::multi => rules::multi(state),
            Rule::divide => rules::divide(state),
            Rule::true_bool => rules::true_bool(state),
            Rule::false_bool => rules::false_bool(state),
            Rule::boolean => rules::boolean(state),
            Rule::is => rules::is(state),
            Rule::not => rules::not(state),
            Rule::truth => rules::truth(state),
            Rule::factor => rules::factor(state),
            Rule::mul => rules::mul(state),
            Rule::expr => rules::expr(state),
            Rule::arg => rules::arg(state),
            Rule::if_stmt => rules::if_stmt(state),
            Rule::while_stmt => rules::while_stmt(state),
            Rule::set_stmt => rules::set_stmt(state),
            Rule::var_stmt => rules::var_stmt(state),
            Rule::program => rules::program(state),
            Rule::call_stmt => rules::call_stmt(state),
            Rule::stmt => rules::stmt(state),
            Rule::stmts => rules::stmts(state),
            Rule::scope => rules::scope(state),
            Rule::file => rules::file(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
