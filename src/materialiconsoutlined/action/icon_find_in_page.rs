
pub struct IconFindInPage {
  props: crate::Props,
}

impl yew::Component for IconFindInPage {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zM6 4h7l5 5v8.58l-1.84-1.84c1.28-1.94 1.07-4.57-.64-6.28C14.55 8.49 13.28 8 12 8c-1.28 0-2.55.49-3.53 1.46-1.95 1.95-1.95 5.11 0 7.05.97.97 2.25 1.46 3.53 1.46.96 0 1.92-.28 2.75-.83L17.6 20H6V4zm8.11 11.1c-.56.56-1.31.88-2.11.88s-1.55-.31-2.11-.88c-.56-.56-.88-1.31-.88-2.11s.31-1.55.88-2.11c.56-.57 1.31-.88 2.11-.88s1.55.31 2.11.88c.56.56.88 1.31.88 2.11s-.31 1.55-.88 2.11z"/></svg>
            </svg>
        }
    }
}

