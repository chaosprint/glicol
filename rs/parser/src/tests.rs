#![cfg(test)]

use crate::{get_ast, nodes::*};

fn ast_from_nodes<const N: usize>(
    nodes: [(&'static str, Vec<Component<'static>>); N]
) -> Result<Ast<'static>, Box<pest::error::Error<crate::Rule>>> {
    Ok(Ast { nodes: hashbrown::HashMap::from_iter(nodes) })
}

#[test]
fn test_arrange() {
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
}
