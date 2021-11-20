
pub struct IconEditOff {
  props: crate::Props,
}

impl yew::Component for IconEditOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><g><path d="M2.1,3.51L2.1,3.51c-0.39,0.39-0.39,1.02,0,1.41l6.61,6.61L3.15,17.1C3.05,17.2,3,17.32,3,17.46v3.04 C3,20.78,3.22,21,3.5,21h3.04c0.13,0,0.26-0.05,0.35-0.15l5.56-5.56l6.61,6.61c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L3.52,3.51C3.12,3.12,2.49,3.12,2.1,3.51z"/></g><g><path d="M20.71,7.04c0.39-0.39,0.39-1.02,0-1.41l-2.34-2.34c-0.39-0.39-1.02-0.39-1.41,0l-1.83,1.83l3.75,3.75L20.71,7.04z"/></g><g><rect height="5.3" transform="matrix(0.7071 -0.7071 0.7071 0.7071 -2.2957 13.1079)" width="3.56" x="12.89" y="6.67"/></g></g></g></svg>
            </svg>
        }
    }
}


