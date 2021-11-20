
pub struct IconAddLocation {
  props: crate::Props,
}

impl yew::Component for IconAddLocation {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M18.5,10.2c0,2.57-2.1,5.79-6.16,9.51L12,20.01l-0.34-0.31 C7.6,15.99,5.5,12.77,5.5,10.2c0-3.84,2.82-6.7,6.5-6.7S18.5,6.35,18.5,10.2z" enable-background="new" opacity=".3"/><g><g><path d="M13,6v3h3v2h-3v3h-2v-3H8V9h3V6H13z M18,10.2C18,6.57,15.35,4,12,4s-6,2.57-6,6.2c0,2.34,1.95,5.44,6,9.14 C16.05,15.64,18,12.54,18,10.2z M12,2c4.2,0,8,3.22,8,8.2c0,3.32-2.67,7.25-8,11.8c-5.33-4.55-8-8.48-8-11.8C4,5.22,7.8,2,12,2z"/></g></g></svg>
            </svg>
        }
    }
}


