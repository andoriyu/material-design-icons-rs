
pub struct IconPunchClock {
  props: crate::Props,
}

impl yew::Component for IconPunchClock {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M19,6h-1V3c0-1.1-0.9-2-2-2H8C6.9,1,6,1.9,6,3v3H5C3.9,6,3,6.9,3,8v12c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V8 C21,6.9,20.1,6,19,6z M8,3h8v3H8V3z M12,19c-2.76,0-5-2.24-5-5s2.24-5,5-5c2.76,0,5,2.24,5,5S14.76,19,12,19z"/><path d="M12.5,13.79V12c0-0.28-0.22-0.5-0.5-0.5h0c-0.28,0-0.5,0.22-0.5,0.5v2c0,0.13,0.05,0.26,0.15,0.35l1.14,1.14 c0.2,0.2,0.51,0.2,0.71,0c0.2-0.2,0.2-0.51,0-0.71L12.5,13.79z"/></g></g></svg>
            </svg>
        }
    }
}


