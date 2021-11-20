
pub struct IconToys {
  props: crate::Props,
}

impl yew::Component for IconToys {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g display="none"><rect display="inline" fill="none" height="24" width="24" y="0"/></g><g><path d="M18.72,10l-2-6H7.28L5.81,8.4L4.41,7l1-1L4,4.59L0.59,8L2,9.41l1-1L4.59,10H2v8h2.18C4.59,19.16,5.7,20,7,20 c1.3,0,2.4-0.84,2.82-2h4.37c0.41,1.16,1.51,2,2.82,2c1.3,0,2.41-0.84,2.82-2H22v-8H18.72z M7,18c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S7.55,18,7,18z M11,10H7.41L7.39,9.98L8.72,6c0,0,0,0,0,0H11V10z M13,10V6h2.28l1.33,4H13z M17,18c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S17.55,18,17,18z"/></g></svg>
            </svg>
        }
    }
}


