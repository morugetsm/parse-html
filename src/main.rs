use std::{rc::Rc, borrow::Borrow};

use html5ever::{serialize, tendril::{TendrilSink, Tendril}, Attribute, QualName, ns, local_name, namespace_url};
use markup5ever_rcdom::{RcDom, SerializableHandle, Node};

fn main() {
    let index_html = include_str!("./index.html");
    let index_string = String::from(index_html);

    let dom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut index_string.as_bytes())
        .unwrap();

    println!("{:?}", dom.quirks_mode);

    walk(&dom.document);

    let document: SerializableHandle = dom.document.clone().into();
    let mut buff = Vec::new();
    serialize(&mut buff, &document, Default::default()).expect("serialization failed");

    let html = String::from_utf8(buff).unwrap();

    println!("\n\n{}", html);
}

fn walk(node: &Rc<Node>) {
    fn internal_walk(parent_node: &Rc<Node>, depth: i32) {
        match parent_node.data.borrow() {
            markup5ever_rcdom::NodeData::Element { name, attrs, template_contents: _, mathml_annotation_xml_integration_point: _ } => {
                print_depth(depth);
                println!("Element <{}> ", name.local);

                let mut attributes = attrs.borrow_mut();

                if attributes.iter().any(|attr| attr.name == QualName::new(None, ns!(), local_name!("id")) && attr.value.as_ref() == "hello") {
                    let attribute = Attribute { name: QualName::new(None, ns!(), local_name!("class")), value: Tendril::from("my-class") };
                    attributes.push(attribute);
                }
            }
            markup5ever_rcdom::NodeData::Text { contents } => {
                print_depth(depth);
                
                let mut contents_ref = contents.borrow_mut();
                if contents_ref.as_ref().find("Nice").is_some() {
                    *contents_ref = Tendril::from("Hello, world!!!");
                }

                let text = contents_ref.as_ref();
                println!("Text {:?}", text)
            }
            _ => {}
        }
        
        for child_node in parent_node.children.borrow().iter() {
            internal_walk(child_node, depth + 1);
        }
    }

    internal_walk(node, 0);
}

fn print_depth(depth: i32) {
    for _ in 1..depth {
        print!("  ");
    }
}