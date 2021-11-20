
pub struct IconImagesearchRoller {
  props: crate::Props,
}

impl yew::Component for IconImagesearchRoller {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M20,7V3c0-0.55-0.45-1-1-1H7C6.45,2,6,2.45,6,3v1H4C2.9,4,2,4.9,2,6v4c0,1.1,0.9,2,2,2h8v3h-1c-0.55,0-1,0.45-1,1v6 c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-6c0-0.55-0.45-1-1-1h-1v-3c0-1.1-0.9-2-2-2H4V6h2v1c0,0.55,0.45,1,1,1h12 C19.55,8,20,7.55,20,7z M8,4h10v2H8V4z M14,21h-2v-4h2V21z"/></g></svg>
            </svg>
        }
    }
}


