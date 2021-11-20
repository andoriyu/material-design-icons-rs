
pub struct IconAssistantDirection {
  props: crate::Props,
}

impl yew::Component for IconAssistantDirection {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M13.5,10H9c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h3.5v1.29c0,0.45,0.54,0.67,0.85,0.35 l2.29-2.29c0.2-0.2,0.2-0.51,0-0.71l-2.29-2.29c-0.31-0.31-0.85-0.09-0.85,0.35V10z M12,1C5.9,1,1,5.9,1,12s4.9,11,11,11 s11-4.9,11-11S18.1,1,12,1z M19.73,12.58l-7.19,7.22c-0.35,0.27-0.79,0.27-1.15,0L4.2,12.58c-0.27-0.36-0.27-0.8,0-1.16l7.19-7.22 c0.35-0.27,0.79-0.27,1.15,0l7.19,7.22C20.09,11.69,20.09,12.22,19.73,12.58z"/></g></g></svg>
            </svg>
        }
    }
}


