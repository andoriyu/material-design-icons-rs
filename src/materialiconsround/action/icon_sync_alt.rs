
pub struct IconSyncAlt {
  props: crate::Props,
}

impl yew::Component for IconSyncAlt {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M21.65,7.65l-2.79-2.79C18.54,4.54,18,4.76,18,5.21V7H4C3.45,7,3,7.45,3,8s0.45,1,1,1h14v1.79c0,0.45,0.54,0.67,0.85,0.35 l2.79-2.79C21.84,8.16,21.84,7.84,21.65,7.65z"/><path d="M20,15H6v-1.79c0-0.45-0.54-0.67-0.85-0.35l-2.79,2.79c-0.2,0.19-0.2,0.51-0.01,0.7l2.79,2.79C5.46,19.46,6,19.24,6,18.79 V17h14c0.55,0,1-0.45,1-1S20.55,15,20,15z"/></g></g></svg>
            </svg>
        }
    }
}


