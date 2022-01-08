use std::collections::HashMap;

use crate::graph::{Atom, Follower};

use super::{Error, JoinPool};

pub fn walk(
    graph: Vec<Atom>,
    follower: &mut impl Follower,
) -> Result<(), Error> {
    let size = graph.len();
    let mut atoms = graph.into_iter().enumerate().collect::<HashMap<_, _>>();
    let mut pool = JoinPool::new();

    for id in 0..size {
        let root = match atoms.remove(&id) {
            Some(root) => root,
            None => continue,
        };

        walk_root(id, root, size, &mut atoms, follower, &mut pool)?;
    }

    Ok(())
}

fn walk_root<F: Follower>(
    pid: usize,
    parent: Atom,
    size: usize,
    atoms: &mut HashMap<usize, Atom>,
    follower: &mut F,
    pool: &mut JoinPool,
) -> Result<(), Error> {
    let mut stack = Vec::new();
    let mut chain = Vec::new();

    for bond in parent.bonds.into_iter().rev() {
        stack.push((pid, bond))
    }

    follower.root(&parent.kind);
    chain.push(pid);

    while let Some((sid, bond)) = stack.pop() {
        if bond.tid >= size {
            return Err(Error::UnknownTarget(sid, bond.tid));
        } else if bond.tid == sid {
            return Err(Error::Loop(sid));
        }

        let mut popcount = 0;

        loop {
            let head = *chain.last().expect("chain head");

            if head == sid {
                break;
            }

            chain.pop();

            popcount += 1
        }

        if popcount > 0 {
            follower.pop(popcount)
        }

        match atoms.remove(&bond.tid) {
            Some(mut child) => {
                let mut back = None;

                for (out_index, out) in
                    child.bonds.into_iter().enumerate().rev()
                {
                    if out.tid == sid {
                        if out_index % 2 == 0 {
                            child.kind.invert_configuration()
                        }

                        if back.is_none() {
                            back = Some(out);
                        } else {
                            return Err(Error::DuplicateBond(sid, bond.tid));
                        }

                        continue;
                    }

                    stack.push((bond.tid, out));
                }

                if let Some(back) = back {
                    if bond.is_directional() {
                        if bond.kind != back.kind.reverse() {
                            return Err(Error::IncompatibleBond(bond.tid, sid));
                        }
                    } else if bond.kind != back.kind {
                        return Err(Error::IncompatibleBond(bond.tid, sid));
                    }
                } else {
                    return Err(Error::HalfBond(sid, bond.tid));
                }

                chain.push(bond.tid);
                follower.extend(&bond.kind, &child.kind)
            }
            None => follower.join(&bond.kind, &pool.hit(sid, bond.tid)),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        graph::{Atom, AtomKind, Bond, BondKind, Bracket, Shortcut, Stereodescriptor, Symbol, VirtualHydrogen},
        walk::Error,
        write::Writer,
    };

    #[test]
    fn half_bond() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Elided, 1)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![],
            },
        ];

        assert_eq!(walk(graph, &mut writer), Err(Error::HalfBond(0, 1)))
    }

    #[test]
    fn duplicate_back_bond() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Elided, 1)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 0),
                ],
            },
        ];

        assert_eq!(walk(graph, &mut writer), Err(Error::DuplicateBond(0, 1)))
    }

    #[test]
    fn unknown_target() {
        let mut writer = Writer::new();
        let graph = vec![Atom {
            kind: AtomKind::Star,
            bonds: vec![Bond::new(BondKind::Elided, 1)],
        }];

        assert_eq!(walk(graph, &mut writer), Err(Error::UnknownTarget(0, 1)))
    }

    #[test]
    fn self_bond() {
        let mut writer = Writer::new();
        let graph = vec![Atom {
            kind: AtomKind::Star,
            bonds: vec![Bond::new(BondKind::Elided, 0)],
        }];

        assert_eq!(walk(graph, &mut writer), Err(Error::Loop(0)))
    }

    #[test]
    fn incompatible_bond() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Elided, 1)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Single, 0)],
            },
        ];

        assert_eq!(walk(graph, &mut writer), Err(Error::IncompatibleBond(1, 0)))
    }

    #[test]
    fn p1() {
        let mut writer = Writer::new();
        let graph = vec![Atom {
            kind: AtomKind::Star,
            bonds: vec![],
        }];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn p2() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Single, 1)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Single, 0)],
            },
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*-*")
    }

    #[test]
    fn p2_directional() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Up, 1)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Down, 0)],
            },
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*/*")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![],
            },
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*.*")
    }

    #[test]
    fn p3() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Single, 1)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Single, 0),
                    Bond::new(BondKind::Single, 2),
                ],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Single, 1)],
            },
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*-*-*")
    }

    #[test]
    fn p3_branched() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![
                    Bond::new(BondKind::Single, 1),
                    Bond::new(BondKind::Double, 2),
                ],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Single, 0)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Double, 0)],
            },
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*(-*)=*")
    }

    #[test]
    fn c3() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Shortcut(Shortcut::C),
                bonds: vec![
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 1),
                ],
            },
            Atom {
                kind: AtomKind::Shortcut(Shortcut::O),
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 2),
                ],
            },
            Atom {
                kind: AtomKind::Shortcut(Shortcut::S),
                bonds: vec![
                    Bond::new(BondKind::Elided, 1),
                    Bond::new(BondKind::Elided, 0),
                ],
            },
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "C(SO1)1")
    }

    #[test]
    fn tetrahedral_root() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Bracket(Bracket {
                    symbol: Symbol::Star,
                    isotope: None,
                    stereodescriptor: Some(Stereodescriptor::Th1),
                    virtual_hydrogen: None,
                    charge: None,
                    extension: None,
                }),
                bonds: vec![
                    Bond::new(BondKind::Elided, 1),
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 4),
                ],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Elided, 0)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Elided, 0)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Elided, 0)],
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![Bond::new(BondKind::Elided, 0)],
            },
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "[*@](*)(*)(*)*")
    }

    #[test]
    fn tetrahedral_child_no_hydrogen() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Bracket(Bracket {
                    symbol: Symbol::Star,
                    isotope: None,
                    stereodescriptor: Some(Stereodescriptor::Th1),
                    virtual_hydrogen: None,
                    charge: None,
                    extension: None,
                }),
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 4)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            }
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*[*@](*)(*)*")
    }

    #[test]
    fn tetrahedral_child_hydrogen() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Bracket(Bracket {
                    symbol: Symbol::Star,
                    isotope: None,
                    stereodescriptor: Some(Stereodescriptor::Th1),
                    virtual_hydrogen: Some(VirtualHydrogen::H),
                    charge: None,
                    extension: None,
                }),
                bonds: vec![
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 4)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            }
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*[*@@H](*)(*)*")
    }

    #[test]
    fn tetrahedral_child_hydrogen_odd_input() {
        let mut writer = Writer::new();
        let graph = vec![
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Bracket(Bracket {
                    symbol: Symbol::Star,
                    isotope: None,
                    stereodescriptor: Some(Stereodescriptor::Th1),
                    virtual_hydrogen: Some(VirtualHydrogen::H),
                    charge: None,
                    extension: None,
                }),
                bonds: vec![
                    Bond::new(BondKind::Elided, 2),
                    Bond::new(BondKind::Elided, 0),
                    Bond::new(BondKind::Elided, 3),
                    Bond::new(BondKind::Elided, 4)
                ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            },
            Atom {
                kind: AtomKind::Star,
                bonds: vec![ Bond::new(BondKind::Elided, 1) ]
            }
        ];

        walk(graph, &mut writer).unwrap();

        assert_eq!(writer.write(), "*[*@H](*)(*)*")
    }
}
