
pub struct IconTimer10Select {
  props: crate::Props,
}

impl yew::Component for IconTimer10Select {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M13,8v8h-3V8H13 M13,5h-3C8.34,5,7,6.34,7,8v8c0,1.66,1.34,3,3,3h3c1.66,0,3-1.34,3-3V8C16,6.34,14.66,5,13,5z M2.5,8H3v9.5 C3,18.33,3.67,19,4.5,19h0C5.33,19,6,18.33,6,17.5V7c0-1.1-0.9-2-2-2H2.5C1.67,5,1,5.67,1,6.5v0C1,7.33,1.67,8,2.5,8z M18.5,11 c-0.83,0-1.5,0.68-1.5,1.5v2c0,0.82,0.67,1.5,1.5,1.5H21v1h-3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h3.5c0.83,0,1.5-0.67,1.5-1.5 v-2c0-0.83-0.67-1.5-1.5-1.5H19v-1h3c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H18.5z"/></svg>
            </svg>
        }
    }
}


