use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::condition::{END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("height", "20px");
}

#[derive(Clone, Debug)]
pub struct MJSpacer {
    attributes: Attributes,
    context: Option<Context>,
}

impl MJSpacer {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSpacer, Error> {
        Ok(MJSpacer {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
        })
    }

    fn set_style_div(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("height", self.get_attribute("height"))
    }
}

impl Component for MJSpacer {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let height = self.get_size_attribute("height");
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .set_style("vertical-align", "top")
            .maybe_set_style("height", height.clone())
            .maybe_set_attribute("height", height.map(|h| h.value()));
        let div = self.set_style_div(Tag::div());
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(td.open());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(div.render("&nbsp;"));
        res.push(START_CONDITIONAL_TAG.into());
        res.push(td.close());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }
}

impl BodyComponent for MJSpacer {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &EMPTY_CHILDREN
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "div" => self.set_style_div(tag),
            _ => tag,
        }
    }
}
