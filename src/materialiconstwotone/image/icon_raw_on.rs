
pub struct IconRawOn {
  props: crate::Props,
}

impl yew::Component for IconRawOn {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M6.5,9H3v6h1.5v-2h1.1l0.9,2H8l-0.9-2.1C7.6,12.6,8,12.1,8,11.5v-1C8,9.7,7.3,9,6.5,9z M6.5,11.5h-2v-1h2V11.5z"/><path d="M10.25,9l-1.5,6h1.5l0.38-1.5h1.75l0.37,1.5h1.5l-1.5-6H10.25z M11,12l0.25-1h0.5L12,12H11z"/><polygon points="19.98,9 19.24,12 18.5,9 16.98,9 16.24,12 15.5,9 14,9 15.5,15 16.98,15 17.74,11.96 18.5,15 19.98,15 21.48,9"/></g></g></svg>
            </svg>
        }
    }
}

