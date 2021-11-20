
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M17.6,10.81L16.19,9.4l3.56-3.55l1.41,1.41C21.05,7.29,17.6,10.81,17.6,10.81z M13,3h-2v5h2V3z M6.4,10.81L7.81,9.4 L4.26,5.84L2.84,7.26C2.95,7.29,6.4,10.81,6.4,10.81z M20,14h-3.42c-0.77,1.76-2.54,3-4.58,3s-3.81-1.24-4.58-3H4v5h16V14 M20,12 c1.1,0,2,0.9,2,2v5c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2v-5c0-1.1,0.9-2,2-2h5c0,1.66,1.34,3,3,3s3-1.34,3-3H20z"/></g></svg>
            </svg>
        }
    }
}


