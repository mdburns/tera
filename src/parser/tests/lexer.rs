use pest::Parser;

use parser::{TeraParser, Rule};

macro_rules! assert_lex_rule {
    ($rule: expr, $input: expr) => {
        let res = TeraParser::parse_str($rule, $input);
        println!("{:?}", $input);
        println!("{:#?}", res);
        if res.is_err() {
            println!("{}", res.unwrap_err());
            panic!();
        }
        assert!(res.is_ok());
        assert_eq!(res.unwrap().last().unwrap().into_span().end(), $input.len());
    };
}

#[test]
fn lex_int() {
    let inputs = vec!["-10", "0", "100", "250000"];
    for i in inputs {
        assert_lex_rule!(Rule::int, i);
    }
}

#[test]
fn lex_float() {
    let inputs = vec!["123.5", "123.5", "0.1", "-1.1"];
    for i in inputs {
        assert_lex_rule!(Rule::float, i);
    }
}

#[test]
fn lex_string() {
    let inputs = vec!["\"Blabla\"", "\"123\""];
    for i in inputs {
        assert_lex_rule!(Rule::string, i);
    }
}

#[test]
fn lex_ident() {
    let inputs = vec!["hello", "hello_", "hello_1", "HELLO", "_1"];
    for i in inputs {
        assert_lex_rule!(Rule::ident, i);
    }

    assert!(TeraParser::parse_str(Rule::ident, "909").is_err());
}

#[test]
fn lex_dotted_ident() {
    let inputs = vec![
        "hello", "hello_", "hello_1", "HELLO", "_1", "hey.ho", "h", "ho",
        "hey.ho.hu", "hey.0", "h.u",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::dotted_ident, i);
    }

    let invalid_inputs = vec![".", "9.w"];
    for i in invalid_inputs {
        assert!(TeraParser::parse_str(Rule::dotted_ident, i).is_err());
    }
}


#[test]
fn lex_expression() {
    let inputs = vec![
        "1 + 1",
        "1 + 2 + 3 * 9/2 + 2.1",
        "index + 1 > 1",
        "show == false",
        "name == \"bob\"",
        "x is defined",
    ];

    for i in inputs {
        assert_lex_rule!(Rule::expression, i);
    }
}

#[test]
fn lex_logic_expression() {
    let inputs = vec![
        // expressions still work
        "1 + 1",
        "1 + 2 + 3 * 9/2 + 2.1",
        "index + 1 > 1",
        "show == false",
        "name == \"bob\"",
        // but also logic one
        "not show",
        "1 > 2 or 3 == 4 and admin",
        "not user_count or true",
        "x > 10 or x is defined",
        "x is defined or x > 10",
    ];

    for i in inputs {
        assert_lex_rule!(Rule::logic_expression, i);
    }
}

#[test]
fn lex_kwarg() {
    let inputs = vec![
        "hello=1",
        "hello=1+1",
        "hello1=true",
        "hello=name",
        "hello=name|filter",
        "hello=name|filter(with_arg=true)",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::kwarg, i);
    }
}

#[test]
fn lex_kwargs() {
    let inputs = vec![
        "hello=1",
        "hello=1+1,hey=1",
        "hello1=true,name=name,admin=true",
        "hello=name",
        "hello=name|filter,id=1",
        "hello=name|filter(with_arg=true),id=1",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::kwargs, i);
    }
}

#[test]
fn lex_fn_call() {
    let inputs = vec![
        "fn(hello=1)",
        "fn(hello=1+1,hey=1)",
        "fn(hello1=true,name=name,admin=true)",
        "fn(hello=name)",
        "fn(hello=name|filter,id=1)",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::fn_call, i);
    }
}

#[test]
fn lex_filter() {
    let inputs = vec![
        "|attr",
        "|attr()",
        "|attr(key=1)",
        "|attr(key=1, more=true)",
        "|attr(key=1,more=true)",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::filter, i);
    }
}

#[test]
fn lex_context_ident() {
    let inputs = vec![
        "hello",
        "hello.hey",
        "hello | attr",
        "hello|attr",
        "hello|attr(key=1)",
        "hello|attr(key=1, more=true)",
        "hello|attr(key=1, more=true)|more",
        "hello|attr(key=1,more=true)|another|more(ok=1)",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::context_ident, i);
    }
}

#[test]
fn lex_macro_definition() {
    let inputs = vec![
        "hello()",
        "hello(name, admin)",
        "hello(name, admin=1)",
        "hello(name=\"bob\", admin)",
        "hello(name=\"bob\",admin=true)",
    ];
    for i in inputs {
        // The () are not counted as tokens for some reasons so can't use the macro
        assert!(TeraParser::parse_str(Rule::macro_fn, i).is_ok());
    }
}

#[test]
fn lex_test() {
    let inputs = vec![
        "a is defined",
        "a is defined()",
        "a is divisibleby(2)",
    ];
    for i in inputs {
        // The () are not counted as tokens for some reasons so can't use the macro
        assert!(TeraParser::parse_str(Rule::test, i).is_ok());
    }
}

#[test]
fn lex_include_tag() {
    assert!(
        TeraParser::parse_str(
            Rule::include_tag,
            "{% include \"index.html\" %}"
        ).is_ok()
    );
}

#[test]
fn lex_import_macro_tag() {
    assert!(
        TeraParser::parse_str(
            Rule::import_macro_tag,
            "{% import \"macros.html\" as macros %}"
        ).is_ok()
    );
}

#[test]
fn lex_extends_tag() {
    assert!(
        TeraParser::parse_str(
            Rule::extends_tag,
            "{% extends \"index.html\" %}"
        ).is_ok()
    );
}

#[test]
fn lex_comment_tag() {
    assert!(
        TeraParser::parse_str(
            Rule::comment_tag,
            "{# #comment# {{}} {%%} #}"
        ).is_ok()
    );
}


#[test]
fn lex_block_tag() {
    let inputs = vec![
        "{% block tag %}",
        "{% block my_block %}",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::block_tag, i);
    }
}

#[test]
fn lex_macro_tag() {
    let inputs = vec![
        "{%- macro tag() %}",
        "{% macro my_block(name) -%}",
        "{% macro my_block(name=42) %}",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::macro_tag, i);
    }
}

#[test]
fn lex_if_tag() {
    let inputs = vec![
        "{%- if name %}",
        "{% if true -%}",
        "{% if admin or show %}",
        "{% if 1 + 2 == 2 and true %}",
        "{% if 1 + 2 == 2 and admin is defined %}",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::if_tag, i);
    }
}

#[test]
fn lex_elif_tag() {
    let inputs = vec![
        "{%- elif name %}",
        "{% elif true -%}",
        "{% elif admin or show %}",
        "{% elif 1 + 2 == 2 and true %}",
        "{% elif 1 + 2 == 2 and admin is defined %}",
    ];
    for i in inputs {
        assert_lex_rule!(Rule::elif_tag, i);
    }
}

#[test]
fn lex_else_tag() {
    assert!(
        TeraParser::parse_str(
            Rule::else_tag,
            "{% else %}"
        ).is_ok()
    );
}

#[test]
fn lex_for_tag() {
    let inputs = vec![
        "{%- for a in array %}",
        "{% for a, b in object -%}",
        "{% for a, b in fn_call() %}",
        "{% for a in fn_call() %}",
        "{% for a,b in fn_call(with_args=true, name=name) %}",
        "{% for client in clients | slice(start=1, end=9) %}",
    ];

    for i in inputs {
        assert_lex_rule!(Rule::for_tag, i);
    }
}

#[test]
fn lex_set_tag() {
    let inputs = vec![
        "{%- set a = true %}",
        "{% set a = object -%}",
        "{% set a = fn_call() %}",
        "{% set a = fn_call(with_args=true, name=name) %}",
        "{% set a = macros::fn_call(with_args=true, name=name) %}",
        "{% set a = var | caps %}",
        "{% set a = var +1 >= 2%}",
    ];

    for i in inputs {
        assert_lex_rule!(Rule::set_tag, i);
    }
}

#[test]
fn lex_variable_tag() {
    let inputs = vec![
        "{{ a }}",
        "{{ a | caps }}",
        "{{ \"hey\" }}",
        "{{ fn_call() }}",
        "{{ macros::fn() }}",
        "{{ name + 42 }}",
        "{{ loop.index + 1 }}",
        "{{ name is defined and name >= 42 }}",
        "{{ my_macros::macro1(hello=\"world\", foo=bar, hey=1+2) }}",
    ];

    for i in inputs {
        assert_lex_rule!(Rule::variable_tag, i);
    }
}

#[test]
fn lex_content() {
    let inputs = vec![
        "some text",
        "{{ name }}",
        "{# comment #}",
        "{% filter upper %}hey{% endfilter %}",
        "{% filter upper() %}hey{% endfilter %}",
        "{% raw %}{{ hey }}{% endraw %}",
        "{% for a in b %}{{a}}{% endfor %}",
        "{% if i18n %}世界{% else %}world{% endif %}",
    ];

    for i in inputs {
        assert_lex_rule!(Rule::content, i);
    }
}

#[test]
fn lex_template() {
    assert!(
        TeraParser::parse_str(
            Rule::template,
            "{# Greeter template #}
            Hello {% if i18n %}世界{% else %}world{% endif %}
            {% for country in countries %}
                {{ loop.index }}.{{ country }}
            {% endfor %}"
        ).is_ok()
    );
}
