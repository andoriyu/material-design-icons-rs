
pub struct IconLineAxis {
  props: crate::Props,
}

impl yew::Component for IconLineAxis {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M21.34,6.77L21.34,6.77c-0.4-0.4-1.07-0.39-1.45,0.04l-3.33,3.74l-5.65-5.24C10.12,4.58,8.9,4.6,8.14,5.36L2.7,10.81 c-0.39,0.39-0.39,1.02,0,1.41l0.09,0.09c0.39,0.39,1.02,0.39,1.41,0l5.44-5.45l5.59,5.19l-1.73,1.95l-2.58-2.58 c-0.78-0.78-2.05-0.78-2.83,0L2.7,16.8c-0.39,0.39-0.39,1.02,0,1.41L2.8,18.3c0.39,0.39,1.02,0.39,1.41,0l5.3-5.3l2.5,2.5 c0.81,0.81,2.14,0.77,2.91-0.09l1.78-2.01l3.19,2.96c0.39,0.36,1,0.35,1.38-0.03l0.01-0.01c0.4-0.4,0.39-1.05-0.03-1.43l-3.22-2.99 l3.35-3.77C21.73,7.74,21.71,7.14,21.34,6.77z"/></g></svg>
            </svg>
        }
    }
}


