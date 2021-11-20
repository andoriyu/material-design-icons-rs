
pub struct IconHdrOn {
  props: crate::Props,
}

impl yew::Component for IconHdrOn {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M7.25 9c-.41 0-.75.34-.75.75V11h-2V9.75c0-.41-.34-.75-.75-.75S3 9.34 3 9.75v4.5c0 .41.34.75.75.75s.75-.34.75-.75V12.5h2v1.75c0 .41.34.75.75.75s.75-.34.75-.75v-4.5C8 9.34 7.66 9 7.25 9zM21 11.5v-1c0-.83-.67-1.5-1.5-1.5H17c-.55 0-1 .45-1 1v4.25c0 .41.34.75.75.75s.75-.34.75-.75V13h1.1l.72 1.59c.11.25.36.41.63.41.5 0 .83-.51.64-.96l-.49-1.14c.5-.3.9-.8.9-1.4zm-3.5 0v-1h2v1h-2zM13 9h-3c-.28 0-.5.22-.5.5v5c0 .28.22.5.5.5h3c.82 0 1.5-.68 1.5-1.5v-3c0-.82-.68-1.5-1.5-1.5zm0 4.5h-2v-3h2v3z"/></svg>
            </svg>
        }
    }
}


