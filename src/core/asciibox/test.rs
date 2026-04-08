#[cfg(test)]
mod tests {
    #[test]
    fn test_xxx() {
        // let mut dag = Graph::new();

        // dag.add_node(1, "Parse");
        // dag.add_node(2, "Compile");
        // dag.add_node(3, "Link");

        // dag.add_edge(1, 2, None); // Parse -> Compile
        // dag.add_edge(2, 3, None); // Compile -> Link

        // println!("{}", dag.render());
    }

    //     #[test]
    //     fn test_abc() {
    //         use ascii_dag::Graph;

    //         let mut g = Graph::new();
    //         g.add_node(1, "Web");
    //         g.add_node(2, "API");
    //         g.add_node(3, "DB");
    //         g.add_node(4, "Cache");

    //         g.add_edge(1, 2, None);
    //         g.add_edge(2, 3, None);
    //         g.add_edge(2, 4, None);

    //         // Create clusters
    //         let frontend = g.add_subgraph("Frontend");
    //         g.put_nodes(&[1]).inside(frontend).unwrap();

    //         let backend = g.add_subgraph("Backend");
    //         g.put_nodes(&[2, 3, 4]).inside(backend).unwrap();

    //         let ir = g.compute_layout();
    //         println!("{}", ir.render_scanline());
    //     }

    //     #[test]
    //     fn test_map_render() {
    //         let mut gmap = AMap::new(true);
    //         let m1code = "a";
    //         let mut result = String::new();
    //         result.push_str(".---.\n");
    //         result.push_str("| a |\n");
    //         result.push_str("'---'\n");
    //         assert_eq!(gmap.load_content(m1code), result);

    //         let mcode = "a[123]";
    //         result = String::new();
    //         result.push_str("+-----+\n");
    //         result.push_str("| 123 |\n");
    //         result.push_str("+-----+\n");
    //         assert_eq!(gmap.load_content(mcode), result);

    //         let mcode = "aa ---> b";
    //         result = String::new();
    //         result.push_str(".----.   .---.\n");
    //         result.push_str("| aa |-->| b |\n");
    //         result.push_str("'----'   '---'\n");
    //         assert_eq!(gmap.load_content(mcode), result);

    //         let mcode = "a-->b-->c --> d";
    //         result = String::new();
    //         result.push_str(".---.   .---.   .---.   .---.\n");
    //         result.push_str("| a |-->| b |-->| c |-->| d |\n");
    //         result.push_str("'---'   '---'   '---'   '---'\n");
    //         assert_eq!(gmap.load_content(mcode), result);

    //         let mcode = "aaa <--- b";
    //         result = String::new();
    //         result.push_str(".-----.   .---.\n");
    //         result.push_str("| aaa |<--| b |\n");
    //         result.push_str("'-----'   '---'\n");
    //         assert_eq!(gmap.load_content(mcode), result);

    //         let mcode = "aba ---v b";
    //         result = String::new();
    //         result.push_str(".-----.\n");
    //         result.push_str("| aba |\n");
    //         result.push_str("'-----'\n");
    //         result.push_str("   |\n");
    //         result.push_str("   v\n");
    //         result.push_str(".-----.\n");
    //         result.push_str("|  b  |\n");
    //         result.push_str("'-----'\n");
    //         assert_eq!(gmap.load_content(mcode), result);

    //         let mcode = "aca ---^ b";
    //         result = String::new();
    //         result.push_str(".-----.\n");
    //         result.push_str("|  b  |\n");
    //         result.push_str("'-----'\n");
    //         result.push_str("   ^\n");
    //         result.push_str("   |\n");
    //         result.push_str(".-----.\n");
    //         result.push_str("| aca |\n");
    //         result.push_str("'-----'\n");
    //         assert_eq!(gmap.load_content(mcode), result);
    //     }

    //     #[test]
    //     fn test_map_group_render() {
    //         let mut gmap = AMap::new(true);
    //         let mut result = String::new();
    //         let code = "b <-- a --> c\n a --^ u\n a --v d";
    //         result.push_str(
    //             "
    //         .---.
    //         | u |
    //         '---'
    //           ^
    //           |
    // .---.   .---.   .---.
    // | b |<--| a |-->| c |
    // '---'   '---'   '---'
    //           |
    //           v
    //         .---.
    //         | d |
    //         '---'
    // ",
    //         );
    //         assert_eq!(gmap.load_content(code), result[1..]);

    //         let code = "
    //         a --> b
    //         a --> c
    //         a --> d
    //         d --> f
    //         f --^ g --> h --^ k";
    //         result = String::new();
    //         result.push_str(
    //             "
    // .---.     .---.           .---.
    // | a |---->| b |           | k |
    // '---'--.  '---'           '---'
    //        |                    ^
    //        |                    |
    //        |  .---.   .---.   .---.
    //        +->| c |   | g |-->| h |
    //        |  '---'   '---'   '---'
    //        |            ^
    //        |            |
    //        |  .---.   .---.
    //        '->| d |-->| f |
    //           '---'   '---'
    // ",
    //         );
    //         assert_eq!(gmap.load_content(code), result[1..]);
    //     }
}
