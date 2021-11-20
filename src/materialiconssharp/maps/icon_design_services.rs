
pub struct IconDesignServices {
  props: crate::Props,
}

impl yew::Component for IconDesignServices {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g/><g><path d="M21.79,17.06l-5.55-5.55l1.57-1.57l-3.75-3.75l-1.57,1.57L6.94,2.21L2.21,6.94l5.55,5.55L3,17.25V21h3.75l4.76-4.76 l5.55,5.55l0,0v0L21.79,17.06z M9.18,11.07L5.04,6.94l1.9-1.9l1.27,1.27L7.02,7.5l1.41,1.41l1.19-1.19l1.45,1.45L9.18,11.07z M12.93,14.82l1.9-1.9l1.45,1.45l-1.19,1.19l1.41,1.41l1.19-1.19l1.27,1.27l-1.9,1.9L12.93,14.82z"/><rect height="5.3" transform="matrix(0.7071 -0.7071 0.7071 0.7071 1.302 14.5981)" width="3.59" x="16.48" y="3.08"/></g></g></svg>
            </svg>
        }
    }
}


