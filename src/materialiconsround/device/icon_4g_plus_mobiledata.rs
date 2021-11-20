
pub struct Icon4gPlusMobiledata {
  props: crate::Props,
}

impl yew::Component for Icon4gPlusMobiledata {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M16,9c0.55,0,1-0.45,1-1s-0.45-1-1-1h-5C9.9,7,9,7.9,9,9v6c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2v-3c0-0.55-0.45-1-1-1h-2 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v2h-4V9H16z"/><path d="M23,11h-1v-1c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h-1c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v1c0,0.55,0.45,1,1,1s1-0.45,1-1v-1h1 c0.55,0,1-0.45,1-1S23.55,11,23,11z"/><path d="M7,12V8c0-0.55-0.45-1-1-1S5,7.45,5,8v4H3V8c0-0.55-0.45-1-1-1S1,7.45,1,8v5c0,0.55,0.45,1,1,1h3v2c0,0.55,0.45,1,1,1 s1-0.45,1-1v-2c0.55,0,1-0.45,1-1S7.55,12,7,12z"/></g></g></svg>
            </svg>
        }
    }
}


