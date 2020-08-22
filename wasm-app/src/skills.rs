use std::collections::HashSet;
use std::iter::FromIterator;
use yew::agent::Bridged;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::protos::resume::SkillCategory;
use crate::tag_agent::TagAgent;

#[derive(Clone, Properties, PartialEq)]
pub struct SkillProps {
    pub skills: Vec<SkillCategory>,
}

pub enum Msg {
    Select(Vec<String>),
    Hover(String),
    Clear,
}

pub struct SkillComponent {
    props: SkillProps,
    _tag_agent: Box<dyn Bridge<TagAgent>>,
    selected_tags: HashSet<String>,
    hovered: Option<String>,
    link: ComponentLink<Self>,
}

impl Component for SkillComponent {
    type Message = Msg;
    type Properties = SkillProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::Select);
        let _tag_agent = TagAgent::bridge(callback);
        Self {
            props,
            _tag_agent,
            selected_tags: HashSet::new(),
            hovered: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(tags) => self.selected_tags = HashSet::from_iter(tags.iter().cloned()),
            Msg::Hover(tag) => self.hovered = Some(tag),
            Msg::Clear => self.hovered = None,
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h2>{ "SELECTED SKILLS" }</h2>
            <ul class="skill-list">
                { for self.props.skills.iter().map(|cat| self.view_category(cat)) }
            </ul>
            </>
        }
    }
}

impl SkillComponent {
    fn view_category(&self, cat: &SkillCategory) -> Html {
        let tags = cat.get_tags().to_vec();
        html! {
            <li>
                <h3>{ cat.get_category() }</h3>
                <div class="tag-container">
                    { for tags.iter().map(|tag| self.view_tag(&tag)) }
                </div>
            </li>
        }
    }

    fn view_tag(&self, tag: &str) -> Html {
        let mut class = if self.selected_tags.contains(&tag.to_string()) {
            "tag-item tag-selected".to_string()
        } else {
            "tag-item".to_string()
        };
        let send_tag = tag.to_string();
        let mouseover = self.link.callback(move |_| Msg::Hover(send_tag.to_owned()));
        let mouseout = self.link.callback(|_| Msg::Clear);
        if let Some(hov_tag) = &self.hovered {
            if hov_tag == tag {
                class = format!("{} tag-selected tag-hover", class);
            }
        }
        html! {
            <div class=class onmouseover=mouseover onmouseout=mouseout>
                { tag }
            </div>
        }
    }
}
