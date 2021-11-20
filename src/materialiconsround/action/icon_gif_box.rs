
pub struct IconGifBox {
  props: crate::Props,
}

impl yew::Component for IconGifBox {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M10.5,10.5c0,0.28-0.22,0.5-0.5,0.5 H8.5v2h1v-0.5c0-0.29,0.25-0.53,0.55-0.5c0.26,0.02,0.45,0.26,0.45,0.52V13c0,0.55-0.45,1-1,1h-1c-0.55,0-1-0.45-1-1v-2 c0-0.55,0.45-1,1-1H10C10.28,10,10.5,10.22,10.5,10.5z M12,10c0.28,0,0.5,0.22,0.5,0.5v3c0,0.28-0.22,0.5-0.5,0.5s-0.5-0.22-0.5-0.5 v-3C11.5,10.22,11.72,10,12,10z M14,14c-0.28,0-0.5-0.22-0.5-0.5v-3c0-0.28,0.22-0.5,0.5-0.5h2c0.28,0,0.5,0.22,0.5,0.5 S16.28,11,16,11h-1.5v0.5h1c0.28,0,0.5,0.22,0.5,0.5s-0.22,0.5-0.5,0.5h-1v1C14.5,13.78,14.28,14,14,14z"/></svg>
            </svg>
        }
    }
}


