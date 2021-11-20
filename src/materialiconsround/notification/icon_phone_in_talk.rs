
pub struct IconPhoneInTalk {
  props: crate::Props,
}

impl yew::Component for IconPhoneInTalk {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M12.88 5.05c3.18.4 5.67 2.89 6.07 6.07.06.51.49.88.99.88.04 0 .08 0 .12-.01.55-.07.94-.57.87-1.12-.51-4.09-3.72-7.3-7.81-7.81-.55-.06-1.05.33-1.11.88-.07.55.32 1.05.87 1.11zm.38 2.11c-.53-.14-1.08.18-1.22.72s.18 1.08.72 1.22c1.05.27 1.87 1.09 2.15 2.15.12.45.52.75.97.75.08 0 .17-.01.25-.03.53-.14.85-.69.72-1.22-.47-1.77-1.84-3.14-3.59-3.59zm5.97 8.1l-2.54-.29c-.61-.07-1.21.14-1.64.57l-1.84 1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85c.43-.43.64-1.03.57-1.64l-.29-2.52c-.12-1.01-.97-1.77-1.99-1.77H5.03c-1.13 0-2.07.94-2 2.07.53 8.54 7.36 15.36 15.89 15.89 1.13.07 2.07-.87 2.07-2v-1.73c.01-1.01-.75-1.86-1.76-1.98z"/></svg>
            </svg>
        }
    }
}

