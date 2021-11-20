
pub struct IconHttp {
  props: crate::Props,
}

impl yew::Component for IconHttp {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M24 24H0V0h24v24z" fill="none" opacity=".87"/><path d="M4.5 11h-2V9.75c0-.41-.34-.75-.75-.75S1 9.34 1 9.75v4.5c0 .41.34.75.75.75s.75-.34.75-.75V12.5h2v1.75c0 .41.34.75.75.75s.75-.34.75-.75v-4.5C6 9.34 5.66 9 5.25 9s-.75.34-.75.75V11zm3.25-.5h.75v3.75c0 .41.34.75.75.75s.75-.34.75-.75V10.5h.75c.41 0 .75-.34.75-.75S11.16 9 10.75 9h-3c-.41 0-.75.34-.75.75s.34.75.75.75zm5.5 0H14v3.75c0 .41.34.75.75.75s.75-.34.75-.75V10.5h.75c.41 0 .75-.34.75-.75S16.66 9 16.25 9h-3c-.41 0-.75.34-.75.75s.34.75.75.75zM21.5 9H19c-.55 0-1 .45-1 1v4.25c0 .41.34.75.75.75s.75-.34.75-.75V13h2c.83 0 1.5-.68 1.5-1.5v-1c0-.82-.67-1.5-1.5-1.5zm0 2.5h-2v-1h2v1z"/></svg>
            </svg>
        }
    }
}


