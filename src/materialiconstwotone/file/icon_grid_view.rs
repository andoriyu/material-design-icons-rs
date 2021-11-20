
pub struct IconGridView {
  props: crate::Props,
}

impl yew::Component for IconGridView {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><rect height="4" opacity=".3" width="4" x="5" y="5"/><rect height="4" opacity=".3" width="4" x="5" y="15"/><rect height="4" opacity=".3" width="4" x="15" y="15"/><rect height="4" opacity=".3" width="4" x="15" y="5"/><path d="M3,21h8v-8H3V21z M5,15h4v4H5V15z"/><path d="M3,11h8V3H3V11z M5,5h4v4H5V5z"/><path d="M13,21h8v-8h-8V21z M15,15h4v4h-4V15z"/><path d="M13,3v8h8V3H13z M19,9h-4V5h4V9z"/></g></g></svg>
            </svg>
        }
    }
}


