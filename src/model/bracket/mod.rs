mod matches;
use std::collections::HashMap;

pub use matches::{PreviousMatchType, TournamentBracketMatch, TournamentPreviousMatch};
use petgraph::Graph;

#[derive(Debug, Clone)]
pub struct TournamentBracket {
    tree: Graph<TournamentBracketMatch, PreviousMatchType>,
}

impl TournamentBracket {
    #[must_use]
    #[inline]
    pub fn new_empty() -> Self {
        Self { tree: Graph::new() }
    }

    #[must_use]
    pub fn new(mut matches: Vec<TournamentBracketMatch>) -> Self {
        let mut graph = Graph::with_capacity(matches.len(), matches.len());

        // This is OK b/c node indices are stable across insertions
        let mut match_map = HashMap::new();

        // Sort matches from oldest
        matches.sort_unstable_by_key(|m| m.inner.scheduled_at);

        for m in matches {
            // Add node to graph
            let id = m.inner.id;
            let node = graph.add_node(m);
            match_map.insert(id, node);

            // Look at previous matches and add edges
            let prev_matches = graph[node].previous_matches.clone();
            for prev_match in prev_matches {
                let prev_node = match_map[&prev_match.match_id];
                graph.add_edge(prev_node, node, prev_match.r#type);
            }
        }

        Self { tree: graph }
    }

    // pub fn add_match(&mut self, m: TournamentBracketMatch) {
    //     todo!()
    // }

    /// Check if the bracket is acyclic.
    #[must_use]
    pub fn is_asyclic(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.tree)
    }

    /// Returns the number of sub-brackets in the bracket.
    ///
    /// A sub-bracket is a set of matches that are weakly-connected to each other.
    /// This can be used to determine the number of brackets within a single tournament.
    ///
    /// # Example
    /// The following tournament has one bracket:
    /// ```text
    ///  ┌───┐
    ///  │ A ├──┐
    ///  └───┘  │ ┌───┐
    ///         ├─┤ E ├──┐
    ///  ┌───┐  │ └───┘  │
    ///  │ B ├──┘        │
    ///  └───┘           │ ┌───┐
    ///                  ├─┤ G │
    ///  ┌───┐           │ └───┘
    ///  │ C ├──┐        │
    ///  └───┘  │ ┌───┐  │
    ///         ├─┤ F ├──┘
    ///  ┌───┐  │ └───┘
    ///  │ D ├──┘
    ///  └───┘
    /// ```
    /// While the following tournament has two brackets:
    /// ```text
    /// ┌───┐
    /// │ A ├──┐
    /// └───┘  │ ┌───┐
    ///        ├─┤ E │
    /// ┌───┐  │ └───┘
    /// │ B ├──┘
    /// └───┘
    /// ┌───┐
    /// │ C ├──┐
    /// └───┘  │ ┌───┐
    ///        ├─┤ F │
    /// ┌───┐  │ └───┘
    /// │ D ├──┘
    /// └───┘
    /// ```
    #[must_use]
    pub fn sub_brackets(&self) -> usize {
        petgraph::algo::connected_components(&self.tree)
    }
}

impl AsRef<Graph<TournamentBracketMatch, PreviousMatchType>> for TournamentBracket {
    fn as_ref(&self) -> &Graph<TournamentBracketMatch, PreviousMatchType> {
        &self.tree
    }
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use super::*;
    use crate::model::matches::CompactMatch;

    macro_rules! make_bracket {
        (
            $(
                $id:literal => $time:literal
                $( => ($from:literal, $tp:ident))*
            ),* $(,)*
        ) => {{
            let mut matches = Vec::<TournamentBracketMatch>::new();
            $(
                let m = TournamentBracketMatch {
                    inner: CompactMatch {
                        id: $id,
                        scheduled_at: Some(time::OffsetDateTime::from_unix_timestamp($time).unwrap()),

                        begin_at: None,
                        detailed_stats: false,
                        draw: false,
                        end_at: None,
                        forfeit: false,
                        game_advantage: None,
                        live: crate::model::matches::MatchLive {
                            opens_at: None,
                            supported: false,
                            url: None,
                        },
                        match_type: crate::model::matches::MatchType::BestOf,
                        modified_at: OffsetDateTime::now_utc(),
                        name: "".into(),
                        number_of_games: 1,
                        original_scheduled_at: None,
                        rescheduled: None,
                        slug: "".into(),
                        status: crate::model::matches::MatchStatus::NotStarted,
                        tournament_id: 0,
                        winner: None,
                    },
                    previous_matches: vec![
                        $(
                            TournamentPreviousMatch {
                                match_id: $from,
                                r#type: PreviousMatchType::$tp,
                            }
                        ),*
                    ],
                    opponents: vec![],
                    results: vec![],
                };
                matches.push(m);
            )*
            TournamentBracket::new(matches)
        }};
    }

    #[test]
    fn test_bracket_new() {
        let _ = make_bracket! {
            1 => 0,
            2 => 1,
            3 => 2 => (1, Winner) => (2, Winner),
        };
    }

    #[test]
    fn test_bracket_sub_brackets() {
        let bracket = make_bracket! {
            1 => 0,
            2 => 1,
            3 => 2 => (1, Winner) => (2, Winner),
        };
        assert_eq!(bracket.sub_brackets(), 1);

        let bracket = make_bracket! {
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4 => (1, Winner) => (2, Winner),
            6 => 5 => (3, Winner) => (4, Winner),
        };
        assert_eq!(bracket.sub_brackets(), 2);
    }
}
