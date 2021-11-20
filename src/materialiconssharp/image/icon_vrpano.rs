
pub struct IconVrpano {
  props: crate::Props,
}

impl yew::Component for IconVrpano {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M12,5.5c-5.25,0-9.01-1.54-10-1.92V20.4c2.16-0.76,5.21-1.9,10-1.9c4.78,0,7.91,1.17,10,1.9V3.6 C19.91,4.33,16.77,5.5,12,5.5z M12,15c-2.34,0-4.52,0.15-6.52,0.41l3.69-4.42l2,2.4L14,10l4.51,5.4C16.52,15.15,14.3,15,12,15z"/></g></svg>
            </svg>
        }
    }
}


