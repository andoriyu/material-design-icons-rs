
pub struct IconBusAlert {
  props: crate::Props,
}

impl yew::Component for IconBusAlert {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><g><path d="M4,11V8h7.29c-0.77-2.6,0.21-4.61,0.37-4.97C2.97,2.67,2,5.02,2,7v9.5c0,0.95,0.38,1.81,1,2.44V22h3v-2h8v2h3v-3.06 c0.62-0.63,1-1.49,1-2.44V13c-1.91,0-3.63-0.76-4.89-2H4z M6.5,17C5.67,17,5,16.33,5,15.5S5.67,14,6.5,14S8,14.67,8,15.5 S7.33,17,6.5,17z M15,15.5c0,0.83-0.67,1.5-1.5,1.5S12,16.33,12,15.5s0.67-1.5,1.5-1.5S15,14.67,15,15.5z"/></g><g><path d="M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,9h-1V8h1V9z M18.5,7h-1V3h1V7z"/></g></g></g></svg>
            </svg>
        }
    }
}


