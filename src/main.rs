use excel_parser::parser::{Token, XMLNode};

fn main() {
    let source = r#"<si>
                            <t>詳細画面レイアウト</t>
                            <rPh>
                                <t>ショウサイ</t>
                            </rPh>
                            <rPh>
                                <t>ガメン</t>
                            </rPh>
                            <phoneticPr/>
                        </si>"#;
    let token_array = Token::create_token_array(source);
    let xml_node = XMLNode::from(token_array);
    println!("{:?}", xml_node)
}
