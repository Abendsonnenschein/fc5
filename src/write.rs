use crate::config::Config;
use crate::entry::ok;
use roxmltree::Node;
use xmlwriter::XmlWriter;

pub fn write(node: Node, cfg: Config) -> String {
    let mut w = XmlWriter::default();
    w.declare();
    write_node(&mut w, node, true, &cfg);
    w.end()
}

fn write_node(w: &mut XmlWriter, node: Node, is_root: bool, cfg: &Config) {
    if !ok(node.tag_name().name(), &node, cfg) {
        return;
    }

    w.open(node.tag_name().name());

    if is_root {
        if let Some(ns) = node.lookup_namespace_uri(Some("exsl")) {
            w.attr("xmlns:exsl", ns);
        }
    }

    for a in node.attributes() {
        w.attr(a.name(), a.value());
    }

    if let Some(text) = node.text() {
        if !text.trim().is_empty() {
            w.text(&text.replace('&', "&amp;"));
            w.close();

            return;
        }
    }

    node.children()
        .filter(|c| c.is_element())
        .for_each(|n| write_node(w, n, false, cfg));

    w.close();
}
