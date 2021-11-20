
pub struct IconFactory {
  props: crate::Props,
}

impl yew::Component for IconFactory {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M14,10V8.48c0-0.71-0.71-1.19-1.37-0.93L9,9V8.52C9,7.8,8.27,7.31,7.61,7.6L3.21,9.48C2.48,9.8,2,10.52,2,11.32V20 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V10H14z M9,17c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2c0-0.55,0.45-1,1-1s1,0.45,1,1V17z M13,17 c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2c0-0.55,0.45-1,1-1s1,0.45,1,1V17z M17,17c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2c0-0.55,0.45-1,1-1 s1,0.45,1,1V17z"/><path d="M20.12,2h-1.23c-0.51,0-0.93,0.38-0.99,0.88L17.2,8.5h4.6l-0.69-5.62C21.05,2.38,20.62,2,20.12,2z"/></g></g></svg>
            </svg>
        }
    }
}


