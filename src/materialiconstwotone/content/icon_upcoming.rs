
pub struct IconUpcoming {
  props: crate::Props,
}

impl yew::Component for IconUpcoming {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M12,17c-2.04,0-3.81-1.24-4.58-3H4v5h16v-5h-3.42C15.81,15.76,14.04,17,12,17z" opacity=".3"/><path d="M21.16,7.26l-1.41-1.41L16.19,9.4l1.41,1.41C17.6,10.81,21.05,7.29,21.16,7.26z"/><rect height="5" width="2" x="11" y="3"/><path d="M20,12h-5c0,1.66-1.34,3-3,3s-3-1.34-3-3H4c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-5 C22,12.9,21.1,12,20,12z M20,19H4v-5h3.42c0.77,1.76,2.54,3,4.58,3s3.81-1.24,4.58-3H20V19z"/><path d="M6.4,10.81L7.81,9.4L4.26,5.84L2.84,7.26C2.95,7.29,6.4,10.81,6.4,10.81z"/></g></g></svg>
            </svg>
        }
    }
}

