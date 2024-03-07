use winnow::{
    ascii::{dec_uint, newline, space0, space1, till_line_ending},
    combinator::repeat,
    error::ErrorKind,
    Parser,
};

type NodeId = u32;

// undirected, no loops
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleGraph {
    nodes: Vec<NodeId>,
    edges: Vec<(NodeId, NodeId)>,
}

impl SimpleGraph {
    /// # Example
    ///
    /// ```text
    /// 2 1
    /// #
    /// 1 First node
    /// 2 Second node
    /// #
    /// 1 2 Edge between the two
    /// ```
    pub fn from_tgf(tgf: &str) -> Self {
        //
        // TODO: parse nodes
        //
        let first_line = (
            space0,
            dec_uint::<_, _, ErrorKind>,
            space1,
            dec_uint,
            space0,
            newline,
        )
            .map(|(_, node_count, _, edge_count, _, _)| (node_count, edge_count));
        let separator = || (space0, "#", space0, newline).map(drop);
        let node = (space0, dec_uint, space1, till_line_ending, space0, newline)
            .map(|(_, node_id, _, _, _, _): (_, u32, _, _, _, _)| node_id);
        let edge = (
            space0,
            dec_uint,
            space1,
            dec_uint,
            till_line_ending,
            space0,
            newline,
        )
            .map(|(_, in_node, _, out_node, _, _, _)| (in_node, out_node));

        fn create_graph(
            ((node_count, edge_count), _sep1, nodes, _sep2, edges): (
                (u32, u32),
                (),
                Vec<NodeId>,
                (),
                Vec<(NodeId, NodeId)>,
            ),
        ) -> SimpleGraph {
            SimpleGraph { nodes, edges }
        }

        let mut tgf_parser = (
            first_line,
            separator(),
            repeat(0.., node),
            separator(),
            repeat(0.., edge),
        )
            .map(create_graph);

        tgf_parser.parse(tgf).unwrap()
    }
}

#[test]
fn parse_simple_graph() {
    let tgf = "  2     1
#
   1 First node
 2 Second node
#
    1       2     Edge between the two
";

    let expected = SimpleGraph {
        nodes: vec![1, 2],
        edges: vec![(1, 2)],
    };

    let parsed_graph = SimpleGraph::from_tgf(tgf);

    assert_eq!(expected, parsed_graph);
}
