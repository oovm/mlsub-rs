use im::{hashset, HashSet};

use crate::auto::{Automaton, StateId};
use crate::{Polarity, TypeSystem};

#[derive(Copy, Clone, Debug)]
pub struct Pair {
    pub neg: StateId,
    pub pos: StateId,
}

#[derive(Default)]
pub(crate) struct FlowSet {
    set: HashSet<StateId>,
}

impl FlowSet {
    pub(crate) fn iter(&self) -> hashset::ConsumingIter<StateId> {
        self.set.clone().into_iter()
    }
}

impl<T: TypeSystem> Automaton<T> {
    pub(crate) fn add_flow(&mut self, pair: Pair) {
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(pair.pos).pol, Polarity::Pos);
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(pair.neg).pol, Polarity::Neg);

        let had_p = self.index_mut(pair.pos).flow.set.insert(pair.neg).is_some();
        let had_n = self.index_mut(pair.neg).flow.set.insert(pair.pos).is_some();
        debug_assert_eq!(had_p, had_n);
    }

    pub(crate) fn remove_flow(&mut self, pair: Pair) {
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(pair.pos).pol, Polarity::Pos);
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(pair.neg).pol, Polarity::Neg);

        let had_p = self
            .index_mut(pair.pos)
            .flow
            .set
            .remove(&pair.neg)
            .is_some();
        let had_n = self
            .index_mut(pair.neg)
            .flow
            .set
            .remove(&pair.pos)
            .is_some();
        debug_assert_eq!(had_p, had_n);
    }

    pub(crate) fn merge_flow_pos(&mut self, pos: StateId, source: StateId) {
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(pos).pol, Polarity::Pos);
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(source).pol, Polarity::Pos);

        for neg in self.index(source).flow.iter() {
            self.add_flow(Pair { pos, neg });
        }
    }

    pub(crate) fn merge_flow_neg(&mut self, neg: StateId, source: StateId) {
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(neg).pol, Polarity::Neg);
        #[cfg(debug_assertions)]
        debug_assert_eq!(self.index(source).pol, Polarity::Neg);

        for pos in self.index(source).flow.iter() {
            self.add_flow(Pair { pos, neg });
        }
    }
}
