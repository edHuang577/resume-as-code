use yew::prelude::*;
use yewtil::NeqAssign;

use crate::protos::resume::Location;

#[derive(Clone, Properties, PartialEq)]
pub struct LocationProps {
    pub location: Location,
}

pub struct LocationComponent {
    props: LocationProps,
}

impl Component for LocationComponent {
    type Message = ();
    type Properties = LocationProps;

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
        let country = self.props.location.get_country().to_ascii_uppercase();
        let country_string = match country.as_str() {
            "US" => "".to_string(),
            _ => format!(", {}", country),
        };
        let city = self.props.location.get_city();
        let state = self.props.location.get_state();
        html! {
            <>
            <i class="fas fa-map-marker-alt"></i>{ format!{"{}, {}{}", city, state, country_string} }
            </>
        }
    }
}
