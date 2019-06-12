use minidom::Element;

pub fn list_item<D: Into<String>>(definition: D, value: Element) -> Element {
    let mut def = Element::bare("strong");
    def.append_text_node(definition);

    let mut space = Element::bare("span");
    space.append_text_node(": ");

    Element::builder("li")
        .append(def)
        .append(space)
        .append(value)
        .build()
}

pub fn span<D: Into<String>>(text: D) -> Element {
    let mut div = Element::bare("span");
    div.append_text_node(text);
    div
}
