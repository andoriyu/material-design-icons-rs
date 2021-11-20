
pub struct IconViewCarousel {
  props: crate::Props,
}

impl yew::Component for IconViewCarousel {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M3,7h2c0.55,0,1,0.45,1,1v8c0,0.55-0.45,1-1,1H3c-0.55,0-1-0.45-1-1V8C2,7.45,2.45,7,3,7z M8,19h8c0.55,0,1-0.45,1-1V6 c0-0.55-0.45-1-1-1H8C7.45,5,7,5.45,7,6v12C7,18.55,7.45,19,8,19z M19,7h2c0.55,0,1,0.45,1,1v8c0,0.55-0.45,1-1,1h-2 c-0.55,0-1-0.45-1-1V8C18,7.45,18.45,7,19,7z"/></svg>
            </svg>
        }
    }
}


