
pub struct IconRowing {
  props: crate::Props,
}

impl yew::Component for IconRowing {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0z" fill="none"/><path d="M4.75 18.25c-.41.41-.41 1.09 0 1.5.41.41 1.09.41 1.5 0L9 17h2l-2.5-2.5-3.75 3.75zM15 5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm5.29 15.3l-2-2.01c-.18-.18-.44-.29-.71-.29H16.5l-6.29-6.29c.79-.33 1.66-.87 2.29-1.39v2.27l3.58 3.58c.57-.55.92-1.32.92-2.16V8.26C17 7.02 15.98 6 14.74 6h-.02c-.34 0-.67.09-.96.23-.26.12-.5.29-.69.5l-1.4 1.55C10.61 9.45 8.66 10.35 7 10.32c-.6 0-1.08.48-1.08 1.08 0 .6.48 1.08 1.08 1.08.31 0 .61-.03.9-.07l7.11 7.09v1.08c0 .26.1.52.29.7l1.99 2.01c.39.39 1.02.39 1.42 0l1.58-1.58c.39-.38.39-1.02 0-1.41z"/></svg>
            </svg>
        }
    }
}


