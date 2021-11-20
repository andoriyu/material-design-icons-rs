
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M14,12c0,0.74-0.4,1.38-1,1.72V22h-2v-8.28c-0.6-0.35-1-0.98-1-1.72c0-1.1,0.9-2,2-2S14,10.9,14,12z M12,6 c-3.31,0-6,2.69-6,6c0,1.74,0.75,3.31,1.94,4.4l1.42-1.42C8.53,14.25,8,13.19,8,12c0-2.21,1.79-4,4-4s4,1.79,4,4 c0,1.19-0.53,2.25-1.36,2.98l1.42,1.42C17.25,15.31,18,13.74,18,12C18,8.69,15.31,6,12,6z M12,2C6.48,2,2,6.48,2,12 c0,2.85,1.2,5.41,3.11,7.24l1.42-1.42C4.98,16.36,4,14.29,4,12c0-4.41,3.59-8,8-8s8,3.59,8,8c0,2.29-0.98,4.36-2.53,5.82l1.42,1.42 C20.8,17.41,22,14.85,22,12C22,6.48,17.52,2,12,2z"/></g></svg>
            </svg>
        }
    }
}


