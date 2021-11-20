
pub struct IconUnpublished {
  props: crate::Props,
}

impl yew::Component for IconUnpublished {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l1.56,1.56 c-1.25,1.88-1.88,4.21-1.59,6.7c0.53,4.54,4.21,8.22,8.74,8.74c2.49,0.29,4.81-0.34,6.7-1.59l1.56,1.56c0.39,0.39,1.02,0.39,1.41,0 l0,0C20.88,21.51,20.88,20.88,20.49,20.49z M9.88,15.89l-2.83-2.83c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0 l2.12,2.12l0.18-0.18l1.41,1.41l-0.88,0.88C10.9,16.28,10.27,16.28,9.88,15.89z M13.59,10.76l-7.1-7.1c1.88-1.25,4.21-1.88,6.7-1.59 c4.54,0.53,8.22,4.21,8.74,8.74c0.29,2.49-0.34,4.82-1.59,6.7l-5.34-5.34l1.94-1.94c0.39-0.39,0.39-1.02,0-1.41v0 c-0.39-0.39-1.02-0.39-1.41,0L13.59,10.76z"/></svg>
            </svg>
        }
    }
}


