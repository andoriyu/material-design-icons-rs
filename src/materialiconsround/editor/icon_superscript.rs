
pub struct IconSuperscript {
  props: crate::Props,
}

impl yew::Component for IconSuperscript {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" x="0" y="0"/><path d="M10.51,12.73L7.3,7.72C6.82,6.97,7.35,6,8.23,6h0c0.39,0,0.74,0.2,0.95,0.53l2.76,4.46h0.12l2.74-4.45 C15,6.2,15.36,6,15.75,6h0c0.88,0,1.42,0.98,0.94,1.72l-3.23,5l3.55,5.55C17.5,19.02,16.96,20,16.08,20h0 c-0.38,0-0.74-0.2-0.95-0.52l-3.07-4.89h-0.12l-3.07,4.89C8.66,19.8,8.31,20,7.92,20h0c-0.88,0-1.42-0.97-0.94-1.72L10.51,12.73z M23,8.5L23,8.5C23,8.22,22.78,8,22.5,8c0,0,0,0,0,0H20V7h2c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h-2.5C19.22,4,19,4.22,19,4.5v0 C19,4.78,19.22,5,19.5,5H22v1h-2c-0.55,0-1,0.45-1,1v1c0,0.55,0.45,1,1,1h2.5C22.78,9,23,8.78,23,8.5z"/></g></svg>
            </svg>
        }
    }
}


