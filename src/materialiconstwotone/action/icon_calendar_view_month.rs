
pub struct IconCalendarViewMonth {
  props: crate::Props,
}

impl yew::Component for IconCalendarViewMonth {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><rect height="5" opacity=".3" width="4" x="4" y="6"/><rect height="5" opacity=".3" width="4" x="4" y="13"/><rect height="5" opacity=".3" width="4" x="10" y="13"/><rect height="5" opacity=".3" width="4" x="16" y="13"/><rect height="5" opacity=".3" width="4" x="16" y="6"/><rect height="5" opacity=".3" width="4" x="10" y="6"/><path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M8,18H4v-5h4V18z M8,11H4V6h4V11z M14,18h-4v-5h4V18z M14,11h-4V6h4V11z M20,18h-4v-5h4V18z M20,11h-4V6h4V11z"/></g></g></svg>
            </svg>
        }
    }
}


