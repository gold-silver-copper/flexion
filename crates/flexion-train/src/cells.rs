//! A part of speech's cells: every group's feature product, numbered in
//! declaration order, and the mapping from Wiktionary tags to cells.

use crate::spec::{Feature, Language, PartOfSpeech};

pub struct Cells<'a> {
    pub groups: Vec<GroupCells<'a>>,
    pub total: usize,
}

pub struct GroupCells<'a> {
    pub name: &'static str,
    pub offset: usize,
    pub features: Vec<&'a Feature>,
    pub group: &'a crate::spec::Group,
}

impl GroupCells<'_> {
    pub fn len(&self) -> usize {
        self.features.iter().map(|f| f.values.len()).product()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The cell index for one value index per feature.
    pub fn index(&self, values: &[usize]) -> usize {
        let mut i = 0;
        for (f, v) in self.features.iter().zip(values) {
            i = i * f.values.len() + v;
        }
        self.offset + i
    }

    /// Value indices of a cell inside this group.
    pub fn values(&self, cell: usize) -> Vec<usize> {
        let mut rest = cell.saturating_sub(self.offset);
        let mut out = vec![0; self.features.len()];
        for (slot, f) in out.iter_mut().zip(&self.features).rev() {
            let n = f.values.len().max(1);
            *slot = rest % n;
            rest /= n;
        }
        out
    }
}

impl<'a> Cells<'a> {
    pub fn new(lang: &'a Language, pos: &'a PartOfSpeech) -> Result<Cells<'a>, String> {
        let mut groups = Vec::new();
        let mut offset = 0;
        for g in &pos.groups {
            let features = g
                .features
                .iter()
                .map(|f| lang.feature(f).ok_or(format!("unknown feature {f}")))
                .collect::<Result<Vec<_>, _>>()?;
            let gc = GroupCells {
                name: g.name,
                offset,
                features,
                group: g,
            };
            offset += gc.len();
            groups.push(gc);
        }
        Ok(Cells {
            groups,
            total: offset,
        })
    }

    pub fn group_of(&self, cell: usize) -> Option<&GroupCells<'a>> {
        self.groups
            .iter()
            .find(|g| cell >= g.offset && cell < g.offset + g.len())
    }

    /// `noun.gen.pl`
    pub fn name(&self, cell: usize) -> String {
        let Some(g) = self.group_of(cell) else {
            return format!("cell{cell}");
        };
        let values = g.features.iter().zip(g.values(cell));
        let shorts = values.filter_map(|(f, v)| Some(f.values.get(v)?.short));
        std::iter::once(g.name)
            .chain(shorts)
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn find(&self, group: &str, values: &[&str]) -> Option<usize> {
        let g = self.groups.iter().find(|g| g.name == group)?;
        if values.len() != g.features.len() {
            return None;
        }
        let idx = g
            .features
            .iter()
            .zip(values)
            .map(|(f, v)| f.values.iter().position(|x| x.name == *v))
            .collect::<Option<Vec<_>>>()?;
        Some(g.index(&idx))
    }

    /// Every cell a form with these tags fills.
    pub fn match_tags(&self, tags: &[String]) -> Vec<usize> {
        self.match_tags_where(tags, &|_| true)
    }

    /// [`Cells::match_tags`], in the groups `wanted` accepts.
    pub fn match_tags_where(
        &self,
        tags: &[String],
        wanted: &dyn Fn(&crate::spec::Group) -> bool,
    ) -> Vec<usize> {
        let has = |t: &str| tags.iter().any(|x| x == t);
        let mut out = Vec::new();
        for g in &self.groups {
            let spec = g.group;
            if !wanted(spec) {
                continue;
            }
            if !spec.all.iter().all(|t| has(t))
                || (!spec.any.is_empty() && !spec.any.iter().any(|t| has(t)))
                || spec.none.iter().any(|t| has(t))
            {
                continue;
            }
            let mut choices: Vec<Vec<usize>> = Vec::new();
            for f in &g.features {
                let mut vals: Vec<usize> = f
                    .values
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.all.iter().all(|t| has(t)) && !v.none.iter().any(|t| has(t)))
                    .map(|(i, _)| i)
                    .collect();
                if vals.is_empty() && f.unspecified_means_all {
                    let mentions_any = f.values.iter().any(|v| v.all.iter().any(|t| has(t)));
                    if !mentions_any {
                        vals = (0..f.values.len()).collect();
                    }
                }
                choices.push(vals);
            }
            if choices.iter().any(Vec::is_empty) {
                continue;
            }
            out.extend(combinations(choices).iter().map(|vals| g.index(vals)));
        }
        out
    }
}

/// Every way of taking one of each of `choices`, in order.
pub fn combinations(choices: impl IntoIterator<Item = Vec<usize>>) -> Vec<Vec<usize>> {
    let mut combos: Vec<Vec<usize>> = vec![Vec::new()];
    for c in choices {
        combos = combos
            .into_iter()
            .flat_map(|prefix| c.iter().map(move |&v| [prefix.as_slice(), &[v]].concat()))
            .collect();
    }
    combos
}
