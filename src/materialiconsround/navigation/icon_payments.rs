
pub struct IconPayments {
  props: crate::Props,
}

impl yew::Component for IconPayments {
 type Properties = crate::Props;
    type Message = ();

    fn create(props: Self::Properties, _: yew::prelude::ComponentLink<Self>) -> Self
    {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> yew::prelude::ShouldRender
    {
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::prelude::ShouldRender
    {
        false
    }

    fn view(&self) -> yew::prelude::Html
    {
        yew::prelude::html! {
            <svg
                class=self.props.class.unwrap_or("")
                width=self.props.size.unwrap_or(24).to_string()
                height=self.props.size.unwrap_or(24).to_string()
                viewBox="0 0 24 24"
                fill=self.props.fill.unwrap_or("none")
                stroke=self.props.color.unwrap_or("currentColor")
                stroke-width=self.props.stroke_width.unwrap_or(2).to_string()
                stroke-linecap=self.props.stroke_linecap.unwrap_or("round")
                stroke-linejoin=self.props.stroke_linejoin.unwrap_or("round")
            >
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0v24h24V0H0z M1,13V7c0-1.66,1.34-3,3-3h12c1.66,0,3,1.34,3,3v7c0,1.1-0.9,2-2,2H4C2.34,16,1,14.66,1,13z M23,18c0,1.1-0.9,2-2,2H5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h16V8c0-0.55,0.45-1,1-1s1,0.45,1,1V18z" fill="none"/><path d="M23,8v10c0,1.1-0.9,2-2,2H5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h16V8c0-0.55,0.45-1,1-1S23,7.45,23,8z M4,16 c-1.66,0-3-1.34-3-3V7c0-1.66,1.34-3,3-3h12c1.66,0,3,1.34,3,3v7c0,1.1-0.9,2-2,2H4z M7,10c0,1.66,1.34,3,3,3s3-1.34,3-3 s-1.34-3-3-3S7,8.34,7,10z"/></g></svg>
            </svg>
        }
    }
}


