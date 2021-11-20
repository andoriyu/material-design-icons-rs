
pub struct IconHevc {
  props: crate::Props,
}

impl yew::Component for IconHevc {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><polygon points="5.5,11 4.5,11 4.5,9 3,9 3,15 4.5,15 4.5,12.5 5.5,12.5 5.5,15 7,15 7,9 5.5,9"/><path d="M21,11v-1c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-1h-1.5v0.5h-1v-3h1V11H21z"/><polygon points="14.25,13.5 13.5,9 12,9 13,15 15.5,15 16.5,9 15,9"/><polygon points="8,9 8,15 11.5,15 11.5,13.5 9.5,13.5 9.5,12.5 11.5,12.5 11.5,11 9.5,11 9.5,10.5 11.5,10.5 11.5,9"/></g></g></svg>
            </svg>
        }
    }
}


