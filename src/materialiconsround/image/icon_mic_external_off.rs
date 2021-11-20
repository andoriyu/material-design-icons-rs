
pub struct IconMicExternalOff {
  props: crate::Props,
}

impl yew::Component for IconMicExternalOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M14,6c0-1.24,1.14-2.22,2.42-1.96C17.36,4.24,18,5.13,18,6.09v9.08l2,2V6.16c0-2.08-1.68-4.03-3.76-4.15 C13.92,1.87,12,3.71,12,6v3.17l2,2V6z"/><path d="M10,5c0-1.66-1.34-3-3-3C6.38,2,5.81,2.19,5.33,2.5l4.15,4.15C9.8,6.18,10,5.61,10,5z"/><path d="M1.39,2.81L1.39,2.81C1,3.2,1,3.83,1.39,4.22L5.17,8H5.1c-0.59,0-1.05,0.51-1,1.1l0.85,8.45C4.98,17.81,5.2,18,5.45,18H6 c0,2.34,2.01,4.21,4.39,3.98c2.08-0.2,3.61-2.06,3.61-4.15l0-1l5.78,5.78c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L2.81,2.81C2.42,2.42,1.78,2.42,1.39,2.81z M12,17.91c0,0.96-0.64,1.86-1.58,2.05 C9.14,20.22,8,19.24,8,18h0.55c0.26,0,0.47-0.19,0.5-0.45l0.52-5.16L12,14.83V17.91z"/></g></g></svg>
            </svg>
        }
    }
}


