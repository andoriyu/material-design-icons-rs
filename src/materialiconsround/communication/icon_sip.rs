
pub struct IconSip {
  props: crate::Props,
}

impl yew::Component for IconSip {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><rect height="1" width="2" x="15.5" y="10.5"/><path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M10,9.75 c0,0.41-0.34,0.75-0.75,0.75H6.5v0.75H9c0.55,0,1,0.45,1,1V14c0,0.55-0.45,1-1,1H5.75C5.34,15,5,14.66,5,14.25v0 c0-0.41,0.34-0.75,0.75-0.75H8.5v-0.75H6c-0.55,0-1-0.45-1-1V10c0-0.55,0.45-1,1-1h3.25C9.66,9,10,9.34,10,9.75L10,9.75z M12,15 L12,15c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v4C13,14.55,12.55,15,12,15z M19,12c0,0.55-0.45,1-1,1h-2.5 v1.25c0,0.41-0.34,0.75-0.75,0.75h0C14.34,15,14,14.66,14,14.25V10c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V12z"/></g></g></svg>
            </svg>
        }
    }
}

