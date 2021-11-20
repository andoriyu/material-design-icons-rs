
pub struct IconSportsMartialArts {
  props: crate::Props,
}

impl yew::Component for IconSportsMartialArts {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M19.06,2.6L11.6,8.7l-1.21-1.04l2.48-1.43c0.57-0.33,0.67-1.11,0.21-1.57l-2.95-2.95c-0.39-0.39-1.02-0.39-1.41,0l0,0 c-0.39,0.39-0.39,1.02,0,1.41l2.03,2.03L5.35,8.26c-0.23,0.13-0.39,0.35-0.46,0.6l-0.96,3.49c-0.07,0.26-0.04,0.53,0.1,0.77 l1.74,3.02c0.28,0.48,0.89,0.64,1.37,0.37h0c0.48-0.28,0.64-0.89,0.37-1.37l-1.53-2.66l0.36-1.29L9.5,13l0.44,8 c0.03,0.56,0.49,1,1.05,1h0c0.56,0,1.02-0.44,1.05-1l0.45-9l7.87-7.96c0.36-0.36,0.38-0.93,0.05-1.32l0,0 C20.07,2.32,19.47,2.27,19.06,2.6z"/><circle cx="5" cy="5" r="2"/></g></g></svg>
            </svg>
        }
    }
}


