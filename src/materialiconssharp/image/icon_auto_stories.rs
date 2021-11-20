
pub struct IconAutoStories {
  props: crate::Props,
}

impl yew::Component for IconAutoStories {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" x="0"/></g><g><path d="M21,4.6v2.12v7.24v2.66C19.86,16.21,18.69,16,17.5,16c-1.9,0-3.78,0.54-5.5,1.58v-3.62V7.79V5.48C10.38,4.55,8.51,4,6.5,4 S2.62,4.55,1,5.48V20c1.52-1.18,3.43-2,5.5-2s3.98,0.82,5.5,2c1.52-1.18,3.43-2,5.5-2s3.98,0.82,5.5,2V5.48 C22.37,5.12,21.7,4.84,21,4.6z"/><polygon points="19,0.5 14,5.5 14,15 19,10.5"/></g></svg>
            </svg>
        }
    }
}


