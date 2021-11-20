
pub struct IconPhishing {
  props: crate::Props,
}

impl yew::Component for IconPhishing {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M17,6.18V2h-2v4.18C13.84,6.6,13,7.7,13,9s0.84,2.4,2,2.82V15c0,2.21-1.79,4-4,4s-4-1.79-4-4v-1.17l1.59,1.59L10,14L5,9v6 c0,3.31,2.69,6,6,6s6-2.69,6-6v-3.18c1.16-0.41,2-1.51,2-2.82S18.16,6.6,17,6.18z M16,10c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S16.55,10,16,10z"/></g></g></svg>
            </svg>
        }
    }
}


