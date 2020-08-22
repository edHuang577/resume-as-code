use yew::prelude::*;
use yewtil::NeqAssign;

use crate::date_range::DateRangeComponent;
use crate::location::LocationComponent;
use crate::protos::resume::{Education, Education_Degree};

impl std::string::ToString for Education_Degree {
    fn to_string(&self) -> String {
        match self {
            Education_Degree::BACHELORS => "B.S.".to_string(),
            Education_Degree::MASTERS => "M.S.".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct EducationProps {
    pub education: Vec<Education>,
}

pub struct EducationComponent {
    props: EducationProps,
}

impl Component for EducationComponent {
    type Message = ();
    type Properties = EducationProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h2>{ "EDUCATION" }</h2>
            <ul class="edu-list">
                { for self.props.education.iter().map(|edu| self.view_entry(edu)) }
            </ul>
            </>
        }
    }
}

impl EducationComponent {
    fn view_entry(&self, edu: &Education) -> Html {
        let title = format!("{} in {}", edu.get_degree().to_string(), edu.get_major());
        let period = edu.get_period().clone();
        let location = edu.get_location().clone();
        let desc = match edu.get_description() {
            x if x != "" => html! { <p>{ format!("{}", x) }</p> },
            _ => html! {},
        };
        html! {
            <li>
                <div class="edu-view">
                    <h3>{ title }</h3>
                    <h4>{ edu.get_institution() }</h4>
                    <div class="detail">
                        <span class="detail-date"><DateRangeComponent period=period/></span>
                        <span class="detail-loc"><LocationComponent location=location/></span>
                    </div>
                    <div class="edu-spec">{ desc }</div>
                </div>
            </li>
        }
    }
}
