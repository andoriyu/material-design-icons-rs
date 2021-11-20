
pub struct IconCallEnd {
  props: crate::Props,
}

impl yew::Component for IconCallEnd {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.59 12.23c.67.38 1.3.8 1.88 1.27l1.07-1.07c-.92-.75-1.91-1.39-2.96-1.91v1.71zM3.53 13.49c.59-.48 1.22-.9 1.87-1.27v-1.7c-1.04.51-2.03 1.15-2.94 1.9l1.07 1.07z" opacity=".3"/><path d="M12 7C7.46 7 3.34 8.78.29 11.67c-.18.18-.29.43-.29.71 0 .28.11.53.29.7l2.48 2.48c.18.18.43.29.71.29.27 0 .52-.1.7-.28.79-.73 1.68-1.36 2.66-1.85.33-.16.56-.51.56-.9v-3.1C8.85 9.25 10.4 9 12 9c1.6 0 3.15.25 4.59.73v3.1c0 .4.23.74.56.9.98.49 1.88 1.11 2.67 1.85.18.17.43.28.7.28.28 0 .53-.11.71-.29l2.48-2.48c.18-.18.29-.43.29-.71 0-.28-.11-.53-.29-.71C20.66 8.78 16.54 7 12 7zm-6.6 5.22c-.65.37-1.28.79-1.87 1.27l-1.07-1.07c.91-.75 1.9-1.38 2.94-1.9v1.7zm15.07 1.28c-.58-.47-1.21-.89-1.88-1.27v-1.71c1.05.51 2.04 1.15 2.96 1.91l-1.08 1.07z"/></svg>
            </svg>
        }
    }
}


