use crate::config::Config;
use roxmltree::Node;

pub fn ok(tag: &str, node: &Node, cfg: &Config) -> bool {
    let overrides = cfg.overrides.clone();

    if let Some(sources) = &cfg.sources.get(tag) {
        return match tag {
            "background" => ok_monster(node, sources, overrides),
            "class" => ok_class(node, overrides),
            "feat" => ok_item(node, sources, overrides),
            "feature" => ok_item(node, sources, overrides),
            "item" => ok_item(node, sources, overrides),
            "monster" => ok_monster(node, sources, overrides),
            "race" => ok_monster(node, sources, overrides),
            "spell" => ok_item(node, sources, overrides),
            _ => true,
        };
    }

    true
}

fn ok_class(node: &Node, overrides: Vec<(String, bool)>) -> bool {
    let name = find_first(node, |n| n.has_tag_name("name"));

    if let Some(name) = name.and_then(|n| n.text()) {
        if let Some(o) = overrides.iter().find(|(n, _)| n == name) {
            return o.1;
        }
    }

    true
}

fn ok_item(node: &Node, sources: &[String], overrides: Vec<(String, bool)>) -> bool {
    let texts: Vec<_> = find_all(node, |n| n.has_tag_name("text"));
    let name = find_first(node, |n| n.has_tag_name("name"));
    let source = texts.iter().find(|n| is_source_text(n.text()));

    if let Some(name) = name.and_then(|n| n.text()) {
        if let Some(o) = overrides.iter().find(|(n, _)| n == name) {
            return o.1;
        }

        if name == "Source" {
            if let Some(source) = texts.last().and_then(|s| s.text()) {
                return sources.iter().any(|s| source.contains(s));
            }
        }
    }

    if let Some(source) = source.and_then(|s| s.text()) {
        return sources.iter().any(|s| source.contains(s));
    }

    false
}

fn ok_monster(node: &Node, sources: &[String], overrides: Vec<(String, bool)>) -> bool {
    let traits: Vec<_> = find_all(node, |n| n.has_tag_name("trait"));
    let name = find_first(node, |n| n.has_tag_name("name"));

    if let Some(name) = name.and_then(|n| n.text()) {
        if let Some(o) = overrides.iter().find(|(n, _)| n == name) {
            return o.1;
        }
    }

    if let Some(first_trait) = traits.first() {
        return ok_item(first_trait, sources, vec![]);
    }

    false
}

fn is_source_text(s: Option<&str>) -> bool {
    match s {
        Some(s) => s.trim().starts_with("Source:"),
        None => false,
    }
}

fn find_all<'a, P>(node: &'a Node, predicate: P) -> Vec<Node<'a, 'a>>
where
    P: Fn(&Node) -> bool,
{
    node.children()
        .filter(|n| n.is_element() && predicate(n))
        .collect()
}

fn find_first<'a, P>(node: &'a Node, predicate: P) -> Option<Node<'a, 'a>>
where
    P: Fn(&Node) -> bool,
{
    node.children().find(|n| n.is_element() && predicate(n))
}
