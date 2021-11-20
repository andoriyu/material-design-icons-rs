
pub struct IconDomainAdd {
  props: crate::Props,
}

impl yew::Component for IconDomainAdd {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M6,19H4v-2h2V19z M6,15H4v-2h2V15z M6,11H4V9h2V11z M6,7H4V5h2V7z M10,19H8v-2h2V19z M10,15H8v-2h2V15z M10,11H8V9h2V11z M10,7H8V5h2V7z M16,19h-4v-2h2v-2h-2v-2h2v-2h-2V9h8v6h2V8c0-0.55-0.45-1-1-1h-9V4c0-0.55-0.45-1-1-1H3C2.45,3,2,3.45,2,4v16 c0,0.55,0.45,1,1,1h13V19z M18,11h-2v2h2V11z M18,15h-2v2h2V15z M24,20c0,0.55-0.45,1-1,1h-1v1c0,0.55-0.45,1-1,1s-1-0.45-1-1v-1 h-1c-0.55,0-1-0.45-1-1s0.45-1,1-1h1v-1c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1v1h1C23.55,19,24,19.45,24,20z"/></g></svg>
            </svg>
        }
    }
}


