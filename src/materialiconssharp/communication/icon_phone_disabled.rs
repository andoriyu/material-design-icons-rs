
pub struct IconPhoneDisabled {
  props: crate::Props,
}

impl yew::Component for IconPhoneDisabled {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><g><g><path d="M14.52,17.35C11.39,19.83,7.36,21.22,3,20.97v-5.51l5.27-0.61l2.52,2.52c0.81-0.41,1.58-0.9,2.3-1.45 L1.39,4.22l1.42-1.41L21.19,21.2l-1.41,1.41L14.52,17.35z M15.91,13.11c0.56-0.73,1.05-1.51,1.47-2.33l-2.53-2.53L15.46,3h5.51 c0.25,4.37-1.15,8.4-3.63,11.54L15.91,13.11z"/></g></g></svg>
            </svg>
        }
    }
}

