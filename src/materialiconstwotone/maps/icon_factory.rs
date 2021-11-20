
pub struct IconFactory {
  props: crate::Props,
}

impl yew::Component for IconFactory {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M12,12l0-2.05l-5,2V10l-3,1.32V20h16v-8H12z M9,18H7v-4h2V18z M13,18h-2v-4h2V18z M17,18h-2v-4h2V18z" opacity=".3"/><path d="M22,22H2V10l7-3v2l5-2l0,3h3l1-8h3l1,8V22z M12,9.95l-5,2V10l-3,1.32V20h16v-8h-8L12,9.95z M11,18h2v-4h-2V18z M7,18h2v-4 H7V18z M17,14h-2v4h2V14z"/></g></svg>
            </svg>
        }
    }
}


