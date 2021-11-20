
pub struct IconPhonelinkSetup {
  props: crate::Props,
}

impl yew::Component for IconPhonelinkSetup {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M7 1v5h2V4h10v16H9v-2H7v5h14V1zm2.5 14.5c.29-.12.55-.29.8-.48l-.02.03 1.41.55 1.27-2.2-1.18-.95-.02.03c.02-.16.05-.32.05-.48s-.03-.32-.05-.48l.02.03 1.18-.95-1.26-2.2-1.41.55.02.03c-.26-.19-.52-.36-.81-.48L9.27 7H6.73L6.5 8.5c-.29.12-.55.29-.8.48l.02-.03L4.3 8.4l-1.27 2.2 1.18.95.02-.03c-.01.16-.04.32-.04.48s.03.32.05.48l-.02-.03-1.18.95 1.27 2.2 1.41-.55-.02-.03c.25.19.51.36.8.48l.23 1.5h2.54l.23-1.5zM6 12c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2z"/></svg>
            </svg>
        }
    }
}


