use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Element;
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;
use prelude::BodyComponent;

pub mod mj_accordion;
pub mod mj_body;
pub mod mj_button;
pub mod mj_carousel;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_group;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_navbar;
pub mod mj_raw;
pub mod mj_section;
pub mod mj_social;
pub mod mj_spacer;
pub mod mj_table;
pub mod mj_text;
pub mod mj_wrapper;
pub mod prelude;
pub mod raw;

#[derive(Clone, Debug)]
pub enum BodyElement {
    MJAccordion(mj_accordion::MJAccordion),
    MJAccordionElement(mj_accordion::MJAccordionElement),
    MJButton(mj_button::MJButton),
    MJCarousel(mj_carousel::MJCarousel),
    MJCarouselImage(mj_carousel::MJCarouselImage),
    MJColumn(mj_column::MJColumn),
    MJDivider(mj_divider::MJDivider),
    MJGroup(mj_group::MJGroup),
    MJHero(mj_hero::MJHero),
    MJImage(mj_image::MJImage),
    MJNavbar(mj_navbar::MJNavbar),
    MJNavbarLink(mj_navbar::MJNavbarLink),
    MJRaw(mj_raw::MJRaw),
    MJSection(mj_section::MJSection),
    MJSocial(mj_social::MJSocial),
    MJSocialElement(mj_social::MJSocialElement),
    MJSpacer(mj_spacer::MJSpacer),
    MJTable(mj_table::MJTable),
    MJText(mj_text::MJText),
    MJWrapper(mj_wrapper::MJWrapper),
    Raw(raw::RawElement),
}

macro_rules! inner_element {
    ($root:expr) => {
        match $root {
            BodyElement::MJAccordion(item) => item,
            BodyElement::MJAccordionElement(item) => item,
            BodyElement::MJButton(item) => item,
            BodyElement::MJCarousel(item) => item,
            BodyElement::MJCarouselImage(item) => item,
            BodyElement::MJColumn(item) => item,
            BodyElement::MJDivider(item) => item,
            BodyElement::MJGroup(item) => item,
            BodyElement::MJHero(item) => item,
            BodyElement::MJImage(item) => item,
            BodyElement::MJNavbar(item) => item,
            BodyElement::MJNavbarLink(item) => item,
            BodyElement::MJRaw(item) => item,
            BodyElement::MJSection(item) => item,
            BodyElement::MJSocial(item) => item,
            BodyElement::MJSocialElement(item) => item,
            BodyElement::MJSpacer(item) => item,
            BodyElement::MJTable(item) => item,
            BodyElement::MJText(item) => item,
            BodyElement::MJWrapper(item) => item,
            BodyElement::Raw(item) => item,
        }
    };
}

impl Component for BodyElement {
    fn update_header(&self, header: &mut Header) {
        self.inner().update_header(header)
    }

    fn context(&self) -> Option<&Context> {
        self.inner().context()
    }

    fn set_context(&mut self, ctx: Context) {
        self.inner_mut().set_context(ctx)
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        self.inner().render(header)
    }
}

impl BodyComponent for BodyElement {
    fn get_children(&self) -> &Vec<BodyElement> {
        self.inner().get_children()
    }
    fn get_current_width(&self) -> Option<Size> {
        self.inner().get_current_width()
    }
    fn attributes(&self) -> Option<&Attributes> {
        self.inner().attributes()
    }
    fn set_style(&self, key: &str, tag: Tag) -> Tag {
        self.inner().set_style(key, tag)
    }

    fn get_width(&self) -> Option<Size> {
        self.inner().get_width()
    }
}

impl BodyElement {
    pub fn inner_mut(&mut self) -> &mut dyn BodyComponent {
        inner_element!(self)
    }

    pub fn inner(&self) -> &dyn BodyComponent {
        inner_element!(self)
    }

    pub fn parse<'a>(
        element: &Element<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<BodyElement, Error> {
        let res = match element {
            Element::Node(node) => match node.name.as_str() {
                "mj-accordion" => {
                    BodyElement::MJAccordion(mj_accordion::MJAccordion::parse(node, header)?)
                }
                "mj-button" => BodyElement::MJButton(mj_button::MJButton::parse(node, header)?),
                "mj-carousel" => {
                    BodyElement::MJCarousel(mj_carousel::MJCarousel::parse(node, header)?)
                }
                "mj-column" => {
                    BodyElement::MJColumn(mj_column::MJColumn::parse(node, header, extra)?)
                }
                "mj-divider" => BodyElement::MJDivider(mj_divider::MJDivider::parse(node, header)?),
                "mj-group" => BodyElement::MJGroup(mj_group::MJGroup::parse(node, header)?),
                "mj-hero" => BodyElement::MJHero(mj_hero::MJHero::parse(node, header)?),
                "mj-image" => BodyElement::MJImage(mj_image::MJImage::parse(node, header)?),
                "mj-navbar" => BodyElement::MJNavbar(mj_navbar::MJNavbar::parse(node, header)?),
                "mj-raw" => BodyElement::MJRaw(mj_raw::MJRaw::parse(node, header)?),
                "mj-section" => BodyElement::MJSection(mj_section::MJSection::parse(node, header)?),
                "mj-social" => BodyElement::MJSocial(mj_social::MJSocial::parse(node, header)?),
                "mj-spacer" => BodyElement::MJSpacer(mj_spacer::MJSpacer::parse(node, header)?),
                "mj-table" => BodyElement::MJTable(mj_table::MJTable::parse(node, header)?),
                "mj-text" => BodyElement::MJText(mj_text::MJText::parse(node, header)?),
                "mj-wrapper" => BodyElement::MJWrapper(mj_wrapper::MJWrapper::parse(node, header)?),
                _ => BodyElement::Raw(raw::RawElement::parse(element, header)?),
            },
            Element::Comment(text) => {
                BodyElement::Raw(raw::RawElement::Comment(text.as_str().to_string()))
            }
            _ => BodyElement::Raw(raw::RawElement::parse(element, header)?),
        };
        Ok(res)
    }

    pub fn is_raw(&self) -> bool {
        matches!(self, BodyElement::Raw(_))
    }
}
