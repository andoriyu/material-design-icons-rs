
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M21.19,21.19L2.81,2.81L1.39,4.22l2.27,2.27C2.61,8.08,2,9.97,2,12c0,5.52,4.48,10,10,10c2.04,0,3.93-0.62,5.51-1.66 l2.27,2.27L21.19,21.19z"/></g><g><path d="M12.34,2.02c-2.18-0.07-4.19,0.55-5.85,1.64l4.59,4.59C10.81,6.2,11.18,4.03,12.34,2.02z"/></g></g></g></svg>
            </svg>
        }
    }
}


