
pub struct IconDonutSmall {
  props: crate::Props,
}

impl yew::Component for IconDonutSmall {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M14.82 11h7.13c-.47-4.72-4.23-8.48-8.95-8.95v7.13c.85.31 1.51.97 1.82 1.82zM15 4.58C17 5.4 18.6 7 19.42 9h-3.43c-.28-.37-.62-.71-.99-.99V4.58zM2 12c0 5.19 3.95 9.45 9 9.95v-7.13C9.84 14.4 9 13.3 9 12c0-1.3.84-2.4 2-2.82V2.05c-5.05.5-9 4.76-9 9.95zm7-7.42v3.44c-1.23.92-2 2.39-2 3.98 0 1.59.77 3.06 2 3.99v3.44C6.04 18.24 4 15.35 4 12c0-3.35 2.04-6.24 5-7.42zm4 10.24v7.13c4.72-.47 8.48-4.23 8.95-8.95h-7.13c-.31.85-.97 1.51-1.82 1.82zm2 1.17c.37-.28.71-.61.99-.99h3.43C18.6 17 17 18.6 15 19.42v-3.43z"/></svg>
            </svg>
        }
    }
}

