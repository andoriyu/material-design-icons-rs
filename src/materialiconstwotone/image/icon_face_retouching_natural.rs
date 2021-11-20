
pub struct IconFaceRetouchingNatural {
  props: crate::Props,
}

impl yew::Component for IconFaceRetouchingNatural {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M10.66,4.12c2.55,4.23,7.03,3.87,7.18,3.86l-0.57-1.25L12.4,4.5l0.85-0.39C12.84,4.04,12.43,4,12,4 C11.54,4,11.09,4.05,10.66,4.12z" opacity=".3"/><path d="M8.08,5.03C6.37,6,5.05,7.58,4.42,9.47C6.13,8.5,7.45,6.92,8.08,5.03z" opacity=".3"/><path d="M19.89,10.75C19.96,11.16,20,11.57,20,12c0,4.41-3.59,8-8,8s-8-3.59-8-8c0-0.05,0.01-0.1,0-0.14 c2.6-0.98,4.69-2.99,5.74-5.55c3.38,4.14,7.97,3.73,8.99,3.61l-0.89-1.93c-0.13,0.01-4.62,0.38-7.18-3.86 c1.01-0.16,1.71-0.15,2.59-0.01l2.12-0.97l0.64-0.29C14.78,2.3,13.43,2,12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10 c0-1.43-0.3-2.78-0.84-4.01L19.89,10.75z M8.08,5.03C7.45,6.92,6.13,8.5,4.42,9.47C5.05,7.58,6.37,6,8.08,5.03z"/><circle cx="9" cy="13" r="1.25"/><circle cx="15" cy="13" r="1.25"/><polygon points="20.6,3.4 19.5,1 18.4,3.4 16,4.5 18.4,5.6 19.5,8 20.6,5.6 23,4.5"/></g></g></svg>
            </svg>
        }
    }
}


