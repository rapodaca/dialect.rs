use crate::follow::Follower;

use crate::tree::{Atom, Child, Target};

pub fn walk(root: &Atom, follower: &mut impl Follower) {
    follower.root(&root.kind);

    let mut stack = children(0, &root);
    let mut depth = 0;

    while let Some((root, push, child)) = stack.pop() {
        if root < depth {
            follower.pop();

            depth = root;
        }

        match child {
            Child::Union(bond_kind, target) => match target {
                Target::Atom(atom) => {
                    depth += 1;

                    if push {
                        follower.push();
                    }

                    stack.append(&mut children(depth, atom));
                    follower.extend(bond_kind, &atom.kind);
                }
                Target::Cut(cut) => follower.join(bond_kind, cut),
            },
            Child::Split(atom) => {
                depth += 1;

                if push {
                    follower.push();
                }

                stack.append(&mut children(depth, atom));
                follower.root(&atom.kind);
            }
        }
    }
}

fn children(root: usize, atom: &Atom) -> Vec<(usize, bool, &Child)> {
    let mut result = Vec::new();

    for (i, child) in atom.children.iter().rev().enumerate() {
        result.push((root, atom.degree() > 1 && i != 0, child));
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{
        feature::{AtomKind, BondKind, Bracket, Cut, Element, Symbol},
        follow::Writer,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn p1() {
        let tree = Atom {
            kind: AtomKind::Star,
            children: vec![],
        };
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn bracket() {
        let tree = Atom {
            kind: AtomKind::Bracket(Bracket {
                symbol: Symbol::Element(Element::Tc),
                ..Default::default()
            }),
            children: vec![],
        };
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "[Tc]");
    }

    #[test]
    fn p2() {
        let tree = Atom {
            kind: crate::feature::AtomKind::Star,
            children: vec![Child::elided_star(vec![])],
        };
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "**")
    }

    #[test]
    fn p2_triple() {
        let tree = Atom {
            kind: crate::feature::AtomKind::Star,
            children: vec![Child::Union(
                BondKind::Triple,
                Target::Atom(Atom::star(vec![])),
            )],
        };
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "*#*")
    }

    #[test]
    fn p3_center() {
        let tree = Atom::star(vec![
            Child::elided_star(vec![]),
            Child::elided_star(vec![]),
        ]);
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "*(*)*")
    }

    #[test]
    fn p2_p1_branch_center() {
        let tree = Atom::star(vec![
            Child::split_star(vec![]),
            Child::elided_star(vec![]),
        ]);
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "*(.*)*")
    }

    #[test]
    fn p3_terminal() {
        let tree = Atom {
            kind: crate::feature::AtomKind::Star,
            children: vec![Child::elided_star(vec![Child::elided_star(
                vec![],
            )])],
        };
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "***")
    }

    #[test]
    fn c3() {
        let tree = Atom::star(vec![
            Child::elided_star(vec![Child::elided_star(vec![
                Child::elided_cut(Cut::C1),
            ])]),
            Child::elided_cut(Cut::C1),
        ]);
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "*(**1)1")
    }

    #[test]
    fn c4() {
        let tree = Atom::star(vec![
            Child::elided_star(vec![Child::elided_star(vec![
                Child::elided_star(vec![Child::elided_cut(Cut::C1)]),
            ])]),
            Child::elided_cut(Cut::C1),
        ]);
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "*(***1)1")
    }

    #[test]
    fn s3_terminal() {
        let tree = Atom::star(vec![Child::elided_star(vec![
            Child::elided_star(vec![]),
            Child::elided_star(vec![]),
        ])]);
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "**(*)*")
    }

    #[test]
    fn s4_terminal() {
        let tree = Atom::star(vec![Child::elided_star(vec![
            Child::elided_star(vec![]),
            Child::elided_star(vec![]),
            Child::elided_star(vec![]),
        ])]);
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "**(*)(*)*")
    }

    #[test]
    fn diamond() {
        let tree = Atom::star(vec![
            Child::elided_star(vec![
                Child::elided_star(vec![Child::elided_star(vec![
                    Child::elided_cut(Cut::C1),
                    Child::elided_cut(Cut::C2),
                ])]),
                Child::elided_cut(Cut::C2),
            ]),
            Child::elided_cut(Cut::C1),
        ]);
        let mut writer = Writer::new();

        walk(&tree, &mut writer);

        assert_eq!(writer.write(), "*(*(**12)2)1")
    }
}
