
pub struct IconPodcasts {
  props: crate::Props,
}

impl yew::Component for IconPodcasts {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M14,12c0,0.74-0.4,1.38-1,1.72V21c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-7.28c-0.6-0.35-1-0.98-1-1.72c0-1.1,0.9-2,2-2 S14,10.9,14,12z M10.75,6.13c-2.27,0.46-4.12,2.28-4.61,4.55c-0.4,1.86,0.07,3.62,1.08,4.94c0.35,0.45,1.03,0.47,1.43,0.07 l0.07-0.07c0.34-0.34,0.34-0.87,0.06-1.25c-0.68-0.9-0.98-2.1-0.66-3.37c0.35-1.42,1.52-2.57,2.95-2.88C13.69,7.52,16,9.49,16,12 c0,0.87-0.28,1.67-0.76,2.32c-0.3,0.41-0.29,0.97,0.07,1.33l0,0c0.44,0.44,1.17,0.37,1.54-0.14C17.57,14.53,18,13.31,18,12 C18,8.28,14.61,5.35,10.75,6.13z M10.83,2.07C6.3,2.58,2.61,6.25,2.07,10.78c-0.35,2.95,0.59,5.67,2.32,7.7 c0.37,0.43,1.03,0.46,1.43,0.06l0.05-0.05c0.35-0.35,0.38-0.92,0.05-1.3c-1.56-1.83-2.33-4.37-1.7-7.06 c0.7-3.01,3.18-5.39,6.22-5.97C15.53,3.18,20,7.08,20,12c0,1.96-0.72,3.76-1.9,5.16c-0.34,0.4-0.31,0.98,0.05,1.35l0,0 c0.42,0.42,1.11,0.39,1.49-0.07C21.11,16.7,22,14.46,22,12C22,6.09,16.87,1.38,10.83,2.07z"/></g></svg>
            </svg>
        }
    }
}

