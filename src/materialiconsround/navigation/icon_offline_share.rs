
pub struct IconOfflineShare {
  props: crate::Props,
}

impl yew::Component for IconOfflineShare {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><g><path d="M5,5L5,5C4.45,5,4,5.45,4,6v15c0,1.1,0.9,2,2,2h9c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6V6C6,5.45,5.55,5,5,5z"/></g><g><path d="M18,1h-8C8.9,1,8,1.9,8,3v14c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V3C20,1.9,19.1,1,18,1z M18,15h-8V5h8V15z"/></g><g><path d="M12.5,10.25h2v0.54c0,0.45,0.54,0.67,0.85,0.35l1.29-1.29c0.2-0.2,0.2-0.51,0-0.71l-1.29-1.29 c-0.31-0.31-0.85-0.09-0.85,0.35v0.54H12c-0.55,0-1,0.45-1,1v1.5c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75V10.25 z"/></g></g></g></svg>
            </svg>
        }
    }
}

