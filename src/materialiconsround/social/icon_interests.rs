
pub struct IconInterests {
  props: crate::Props,
}

impl yew::Component for IconInterests {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M7.02,13c-2.21,0-4,1.79-4,4s1.79,4,4,4s4-1.79,4-4S9.23,13,7.02,13z M13,14v6c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-6 c0-0.55-0.45-1-1-1h-6C13.45,13,13,13.45,13,14z M6.13,3.57l-3.3,5.94C2.46,10.18,2.94,11,3.7,11h6.6c0.76,0,1.24-0.82,0.87-1.49 l-3.3-5.94C7.49,2.89,6.51,2.89,6.13,3.57z M19.25,2.5c-1.06,0-1.81,0.56-2.25,1.17c-0.44-0.61-1.19-1.17-2.25-1.17 C13.19,2.5,12,3.78,12,5.25c0,1.83,2.03,3.17,4.35,5.18c0.37,0.32,0.92,0.32,1.3,0C19.97,8.42,22,7.08,22,5.25 C22,3.78,20.81,2.5,19.25,2.5z"/></svg>
            </svg>
        }
    }
}

