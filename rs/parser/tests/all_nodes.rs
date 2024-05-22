#![cfg(test)]

use glicol_parser::{get_ast, Rule, nodes::*};
use pest::error::{Error, ErrorVariant};

fn ast_from_nodes<const N: usize>(
    nodes: [(&'static str, Vec<Component<'static>>); N]
) -> Result<Ast<'static>, Box<Error<Rule>>> {
    Ok(Ast { nodes: hashbrown::HashMap::from_iter(nodes) })
}

// TODO: Write test for Component::Points parsing

#[test]
fn delay() {
    assert_eq!(
        get_ast("o: delayn 8"),
        ast_from_nodes([
            ("o", vec![Component::Delayn(Delayn {
                param: UsizeOrRef::Usize(8)
            })])
        ])
    );

    assert_eq!(
        get_ast("o: delayn o"),
        ast_from_nodes([
            ("o", vec![Component::Delayn(Delayn {
                param: UsizeOrRef::Ref("o")
            })])
        ])
    );

    assert_eq!(
        match get_ast("o: delayn 0.5").unwrap_err().variant {
            ErrorVariant::ParsingError { positives, .. } => positives,
            _ => unreachable!()
        },
        vec![Rule::integer]
    );

    assert_eq!(
        get_ast("o: delayms 0.5"),
        ast_from_nodes([
            ("o", vec![Component::Delayms(Delayms {
                param: NumberOrRef::Number(0.5)
            })])
        ])
    );

    assert_eq!(
        get_ast("o: delayms 5"),
        ast_from_nodes([
            ("o", vec![Component::Delayms(Delayms {
                param: NumberOrRef::Number(5.)
            })])
        ])
    );

    assert_eq!(
        get_ast("o: delayms o"),
        ast_from_nodes([
            ("o", vec![Component::Delayms(Delayms {
                param: NumberOrRef::Ref("o")
            })])
        ])
    );
}

#[test]
fn waves() {
    assert_eq!(
        get_ast("o: sin 0.5"),
        ast_from_nodes([
            ("o", vec![Component::Sin(Sin {
                param: NumberOrRef::Number(0.5)
            })])
        ])
    );

    assert_eq!(
        get_ast("o: sin i"),
        ast_from_nodes([
            ("o", vec![Component::Sin(Sin {
                param: NumberOrRef::Ref("i")
            })])
        ])
    );

    assert_eq!(
        get_ast("o: squ 1100.5"),
        ast_from_nodes([
            ("o", vec![Component::Squ(Squ {
                param: NumberOrRef::Number(1100.5)
            })])
        ])
    );

    assert_eq!(
        get_ast("o: squ suq"),
        ast_from_nodes([
            ("o", vec![Component::Squ(Squ {
                param: NumberOrRef::Ref("suq")
            })])
        ])
    );

    assert_eq!(
        get_ast("o: saw 00.5"),
        ast_from_nodes([
            ("o", vec![Component::Saw(Saw {
                param: NumberOrRef::Number(0.5)
            })])
        ])
    );

    assert_eq!(
        get_ast("o: saw ooooo"),
        ast_from_nodes([
            ("o", vec![Component::Saw(Saw {
                param: NumberOrRef::Ref("ooooo")
            })])
        ])
    );
}

#[test]
fn seq() {
    assert_eq!(
        get_ast("o: seq 60_ 1000_ 1010__10 _1010_1011_ 1_1_ ~a12_13__ r4 4"),
        ast_from_nodes([
            ("o", vec![Component::Seq(Seq {
                events: vec![
                    (0., NumberOrRef::Number(60.)),
                    (1., NumberOrRef::Number(1000.)),
                    (2., NumberOrRef::Number(1010.)),
                    (2.75, NumberOrRef::Number(10.)),
                    (3.2, NumberOrRef::Number(1010.)),
                    (3.4, NumberOrRef::Number(1011.)),
                    (4., NumberOrRef::Number(1.)),
                    (4.5, NumberOrRef::Number(1.)),
                    (5., NumberOrRef::Ref("~a12")),
                    (5.4, NumberOrRef::Number(13.)),
                    (6., NumberOrRef::Ref("r4")),
                    (7., NumberOrRef::Number(4.))
                ]
            })])
        ])
    );
}

#[test]
fn arrange() {
    assert_eq!(
        get_ast("o: arrange ~o 1"),
        ast_from_nodes([
            ("o", vec![Component::Arrange(Arrange {
                events: vec![
                    NumberOrRef::Ref("~o"),
                    NumberOrRef::Number(1.)
                ]
            })])
        ])
    );

	assert_eq!(
		get_ast("o: arrange ~t1 3 ~t2 1"),
		ast_from_nodes([
			("o", vec![Component::Arrange(Arrange {
				events: vec![
					NumberOrRef::Ref("~t1"),
					NumberOrRef::Number(3.),
					NumberOrRef::Ref("~t2"),
					NumberOrRef::Number(1.)
				]
			})])
		])
	);
}

#[test]
fn choose() {
	assert_eq!(
		get_ast("~a: choose 42 42 42 42 42 37 0 0 0 0"),
		ast_from_nodes([
			("~a", vec![Component::Choose(Choose {
				choices: vec![42., 42., 42., 42., 42., 37., 0., 0., 0., 0.]
			})])
		])
	);

	assert_eq!(
		get_ast("o: choose 52"),
		ast_from_nodes([
			("o", vec![Component::Choose(Choose {
				choices: vec![52.]
			})])
		])
	);
}

#[test]
fn mix() {
	assert_eq!(
		get_ast("out: mix ~bd ~sn ~hh ~lead ~basslow ~bassmid"),
		ast_from_nodes([
			("out", vec![Component::Mix(Mix {
				nodes: vec!["~bd", "~sn", "~hh", "~lead", "~basslow", "~bassmid"]
			})])
		])
	);

	assert_eq!(
		get_ast("out: mix ~t.. ~drum.."),
		ast_from_nodes([
			("out", vec![Component::Mix(Mix {
				nodes: vec!["~t..", "~drum.."]
			})])
		])
	);
}

#[test]
fn sp() {
	assert_eq!(
		get_ast("o: sp \\808db"),
		ast_from_nodes([
			("o", vec![Component::Sp(Sp {
				sample_sym: "\\808db"
			})])
		])
	);

	assert_eq!(
		get_ast("o: sp \\guitar"),
		ast_from_nodes([
			("o", vec![Component::Sp(Sp {
				sample_sym: "\\guitar"
			})])
		])
	);

	assert_eq!(
		get_ast("o: sp \\##s(\"in\")#"),
		ast_from_nodes([
			("o", vec![Component::Sp(Sp {
				sample_sym: "\\##s(\"in\")#"
			})])
		])
	);
}
