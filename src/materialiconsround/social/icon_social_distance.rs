
pub struct IconSocialDistance {
  props: crate::Props,
}

impl yew::Component for IconSocialDistance {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M4,5c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S4,6.1,4,5z M8.78,8.58C7.93,8.21,6.99,8,6,8S4.07,8.21,3.22,8.58 C2.48,8.9,2,9.62,2,10.43L2,11h8l0-0.57C10,9.62,9.52,8.9,8.78,8.58z M18,7c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2 C16,6.1,16.9,7,18,7z M20.78,8.58C19.93,8.21,18.99,8,18,8c-0.99,0-1.93,0.21-2.78,0.58C14.48,8.9,14,9.62,14,10.43L14,11h8l0-0.57 C22,9.62,21.52,8.9,20.78,8.58z M21.65,16.65l-2.79-2.79C18.54,13.54,18,13.76,18,14.21V16H6v-1.79c0-0.45-0.54-0.67-0.85-0.35 l-2.79,2.79c-0.2,0.2-0.2,0.51,0,0.71l2.79,2.79C5.46,20.46,6,20.24,6,19.79V18h12v1.79c0,0.45,0.54,0.67,0.85,0.35l2.79-2.79 C21.84,17.16,21.84,16.84,21.65,16.65z"/></svg>
            </svg>
        }
    }
}


