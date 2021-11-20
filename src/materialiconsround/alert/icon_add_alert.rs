
pub struct IconAddAlert {
  props: crate::Props,
}

impl yew::Component for IconAddAlert {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M12,22c1.1,0,2-0.9,2-2h-4C10,21.1,10.9,22,12,22z"/></g><g><path d="M19,17h-1v-7c0-2.79-1.91-5.14-4.5-5.8V3.5C13.5,2.67,12.83,2,12,2s-1.5,0.67-1.5,1.5v0.7C7.91,4.86,6,7.21,6,10v7H5 c-0.55,0-1,0.45-1,1s0.45,1,1,1h14c0.55,0,1-0.45,1-1S19.55,17,19,17z M14,13h-1v1c0,0.55-0.45,1-1,1s-1-0.45-1-1v-1h-1 c-0.55,0-1-0.45-1-1s0.45-1,1-1h1v-1c0-0.55,0.45-1,1-1s1,0.45,1,1v1h1c0.55,0,1,0.45,1,1S14.55,13,14,13z"/></g></g></g></svg>
            </svg>
        }
    }
}


