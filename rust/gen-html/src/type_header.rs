use crate::body_writer::BodyWriter;
use crate::body_writer::MaybeElementWriter;
use crate::type_description::type_description;
use crate::body_writer::Context;
use minidom::Element;
use liquesco_schema::identifier::Format;

struct TypeHeader<Tg>;

impl<Tg> BodyWriter for TypeHeader<Tg> {
    type T = Tg;

    fn write(ctx: &mut Context<Self::T>) -> Element {
        // name / title
        let mut header_title = Element::builder("div").attr("class", "liquesco-type-header-title");
        let name = ctx.display_name();
        let mut title = Element::builder("h1").build();
        title.append_text_node(name.to_string(Format::SnakeCase));
        header_title = header_title.append(title);

        let mut header_body = Element::builder("div").attr("class", "liquesco-type-header-body");

        // type info
        let (type_name, type_description) = type_description(ctx.type_info().any_type());
        let mut type_info_elem = Element::builder("p")
            .attr("class", "liquesco-type-info")
            .build();
        let mut type_name_elem = Element::bare("em");
        type_name_elem.append_text_node(type_name);
        type_info_elem.append_child(type_name_elem);
        type_info_elem.append_text_node(": ");
        type_info_elem.append_text_node(type_description);
        header_body = header_body.append(type_info_elem);

        // description?
        if let Some(description) = ctx.type_info().any_type().meta().doc() {
            let mut description_elem = Element::builder("p")
                .attr("class", "liquesco-description")
                .build();
            description_elem.append_text_node(description);
            header_body = header_body.append(description_elem);
        }

        let anchor_id = ctx.self_anchor_id();
        Element::builder("div")
            .attr("id", anchor_id)
            .append(header_title)
            .append(header_body.build())
            .build()
    }
}

struct TypeFooter<T>;

impl<Tg> MaybeElementWriter for TypeFooter<Tg> {
    type T = Tg;

    fn write(ctx: &Context<Self::T>) -> Option<Element> {


        let used_by = ctx.usage.is_used_by(&ctx.type_info().reference());
        if !used_by.is_empty() {
            let mut used_by_element = Element::builder("div")
                .attr("class", "liquesco-type-footer")
                .build();
            let mut text = Element::bare("p");
            text.append_text_node("This type is used by:");
            used_by_element.append_child(text);

            let mut ul = Element::bare("ul");
            for used_by_item in used_by {
                let link = ctx.link_to(Some(used_by_item))?;
                let mut li = Element::bare("li");
                li.append_child(link);
                ul.append_child(li);
            }
            used_by_element.append_child(ul);
            Some(used_by_element)
        } else {
            None
        }
    }
}