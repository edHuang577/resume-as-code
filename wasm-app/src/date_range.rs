use std::string::ToString;

use chrono::NaiveDateTime;
use protobuf::well_known_types::Timestamp;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::protos::resume::DateRange;

pub trait DateFormat {
    fn format_month(&self) -> String;
}

impl DateFormat for Timestamp {
    fn format_month(&self) -> String {
        let ndt = NaiveDateTime::from_timestamp(self.get_seconds(), 0);
        ndt.format("%b %Y").to_string()
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct DateRangeProps {
    pub period: DateRange,
}

pub struct DateRangeComponent {
    props: DateRangeProps,
}

impl Component for DateRangeComponent {
    type Message = ();
    type Properties = DateRangeProps;

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
        let start = self.props.period.get_start().format_month();
        let end = if self.props.period.has_end() {
            self.props.period.get_end().format_month()
        } else {
            "Present".to_string()
        };
        html! {
            <>
            <i class="far fa-calendar-alt"></i>{ start }{ " – "}{ end }
            </>
        }
    }
}
