#[cfg(test)]
pub mod xml_node_test {
    use crate::xml::nodes::node::XMLNode;
    #[test]
    fn get_nth_child_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let mut root_node = XMLNode::from(data);
        let child = root_node.nth_child(0).unwrap();
        assert_eq!(
            child,
            XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
            <div/>"#
            )
        );
        let child = root_node.nth_child(0);
        assert_eq!(child, None);
        let child = root_node.nth_child(0);
        assert_eq!(child, None);
    }
    #[test]
    fn search_node_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let search_node = root_node.search_node("div").unwrap().clone();
        assert_eq!(
            search_node,
            XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
            <div/>"#
            )
        );
        let search_node = search_node.search_node("p").unwrap();
        assert_eq!(search_node, &XMLNode::from(r#"<p>p-data</p>"#));
    }
    #[test]

    fn search_nodes_test() {
        let data = r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <p>p-data-2</p>
            <data/>
            div-data</div>
        </div>"#;
        let root_node = XMLNode::from(data);
        let search_node = root_node.search_nodes("div").unwrap();
        assert_eq!(
            search_node,
            vec![&XMLNode::from(
                r#"<div>div-first
            <p>p-data</p>
            <p>p-data-2</p>
            <data/>
            div-data</div>
            <div/>"#
            )]
        );
        let search_node = search_node[0];
        let search_node = search_node.search_nodes("p").unwrap();
        assert_eq!(
            search_node,
            vec![
                &XMLNode::from(r#"<p>p-data</p>"#),
                &XMLNode::from(r#"<p>p-data-2</p>"#)
            ]
        );
    }
    #[test]
    fn element_all_test() {
        let data = r#"<div id="1180" name="kai">
            <p class="p1">p-data</p>
            <p class="p1">p-data-2</p>
            <data/>
        </div>"#;
        let mut root_node = XMLNode::from(data);
        assert_eq!(
            root_node.element_all("class", "p1"),
            Some(vec![
                &XMLNode::from(r#"<p class="p1">p-data</p>"#),
                &XMLNode::from(r#"<p class="p1">p-data-2</p>"#)
            ])
        );
    }
}
