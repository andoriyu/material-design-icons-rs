
pub struct IconBedtimeOff {
  props: crate::Props,
}

impl yew::Component for IconBedtimeOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M9.27,4.49C9.14,5.08,9.07,5.64,9.03,6.2l2.05,2.05c-0.27-2.05,0.1-4.22,1.26-6.23c-0.12,0-0.23-0.01-0.35-0.01 c-2.05,0-3.93,0.61-5.5,1.65l1.46,1.46C8.37,4.88,8.81,4.66,9.27,4.49z"/><path d="M1.39,4.22l2.27,2.27C2.61,8.07,2,9.97,2,12c0,5.52,4.48,10,10,10c2.04,0,3.92-0.63,5.5-1.67l2.28,2.28l1.41-1.41 L2.81,2.81L1.39,4.22z M5.13,7.96l10.92,10.92C14.84,19.6,13.45,20,12,20c-4.41,0-8-3.59-8-8C4,10.52,4.42,9.15,5.13,7.96z"/></g></g></svg>
            </svg>
        }
    }
}


