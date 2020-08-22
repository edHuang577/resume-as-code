use yew::prelude::*;
use yewtil::NeqAssign;

use crate::protos::resume::PhoneNumber;

#[derive(Clone, Properties, PartialEq)]
pub struct PhoneNumberProps {
    pub phone: PhoneNumber,
}

pub struct PhoneNumberComponent {
    props: PhoneNumberProps,
}

impl Component for PhoneNumberComponent {
    type Message = ();
    type Properties = PhoneNumberProps;

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
        let mut phone = String::new();
        for (i, c) in self
            .props
            .phone
            .get_number()
            .to_string()
            .chars()
            .rev()
            .enumerate()
        {
            if (i == 4) | (i == 7) {
                phone.insert(0, '-');
            }
            phone.insert(0, c);
        }
        html! {
            <>
            <i class="fas fa-phone"></i>{ format!("+{} {}", self.props.phone.get_country_code(), phone) }
            </>
        }
    }
}
