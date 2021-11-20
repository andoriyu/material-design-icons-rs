
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M20,3v4c0,0.55-0.45,1-1,1H7C6.45,8,6,7.55,6,7V6H4v4h8c1.1,0,2,0.9,2,2v3h1c0.55,0,1,0.45,1,1v6c0,0.55-0.45,1-1,1h-4 c-0.55,0-1-0.45-1-1v-6c0-0.55,0.45-1,1-1h1v-3H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h2V3c0-0.55,0.45-1,1-1h12 C19.55,2,20,2.45,20,3z"/></g></g></svg>
            </svg>
        }
    }
}


