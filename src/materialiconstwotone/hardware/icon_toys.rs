
pub struct IconToys {
  props: crate::Props,
}

impl yew::Component for IconToys {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g display="none"><rect display="inline" fill="none" height="24" width="24" y="0"/></g><g><g><path d="M18,12H6c-1.1,0-2,0.9-2,2c0,0.51,0.2,0.97,0.51,1.32C5.05,14.53,5.97,14,7,14c1.3,0,2.4,0.84,2.82,2h4.37 c0.41-1.16,1.51-2,2.82-2c1.03,0,1.95,0.53,2.49,1.32C19.8,14.97,20,14.51,20,14C20,12.9,19.1,12,18,12z" opacity=".3"/><path d="M18.75,10.08L17.4,6.05C17,4.82,15.85,4,14.56,4H9.44C8.15,4,7,4.82,6.6,6.05L5.81,8.4L4.41,7l0.29-0.29 c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0l-2,2c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0L3,8.41 l1.79,1.79C3.18,10.72,2,12.22,2,14c0,1.49,0.83,2.78,2.05,3.47C4.27,18.9,5.51,20,7,20c1.3,0,2.4-0.84,2.82-2h4.37 c0.41,1.16,1.51,2,2.82,2c1.49,0,2.73-1.1,2.95-2.53C21.17,16.78,22,15.49,22,14C22,12.05,20.6,10.43,18.75,10.08z M13,6h1.56 c0.43,0,0.81,0.27,0.95,0.68L16.61,10H13V6z M8.49,6.68C8.63,6.27,9.01,6,9.44,6H11v4H7.41L7.39,9.98L8.49,6.68z M7,18 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S7.55,18,7,18z M17,18c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S17.55,18,17,18z M19.49,15.32C18.95,14.53,18.03,14,17,14c-1.3,0-2.4,0.84-2.82,2H9.82C9.4,14.84,8.3,14,7,14c-1.03,0-1.95,0.53-2.49,1.32 C4.2,14.97,4,14.51,4,14c0-1.1,0.9-2,2-2h12c1.1,0,2,0.9,2,2C20,14.51,19.8,14.97,19.49,15.32z"/></g></g></svg>
            </svg>
        }
    }
}


