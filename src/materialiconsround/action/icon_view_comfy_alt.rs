
pub struct IconViewComfyAlt {
  props: crate::Props,
}

impl yew::Component for IconViewComfyAlt {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M10.5,17h-3 C7.22,17,7,16.78,7,16.5v-3C7,13.22,7.22,13,7.5,13h3c0.28,0,0.5,0.22,0.5,0.5v3C11,16.78,10.78,17,10.5,17z M10.5,11h-3 C7.22,11,7,10.78,7,10.5v-3C7,7.22,7.22,7,7.5,7h3C10.78,7,11,7.22,11,7.5v3C11,10.78,10.78,11,10.5,11z M16.5,17h-3 c-0.28,0-0.5-0.22-0.5-0.5v-3c0-0.28,0.22-0.5,0.5-0.5h3c0.28,0,0.5,0.22,0.5,0.5v3C17,16.78,16.78,17,16.5,17z M16.5,11h-3 c-0.28,0-0.5-0.22-0.5-0.5v-3C13,7.22,13.22,7,13.5,7h3C16.78,7,17,7.22,17,7.5v3C17,10.78,16.78,11,16.5,11z"/></g></g></svg>
            </svg>
        }
    }
}


