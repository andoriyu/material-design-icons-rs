
pub struct IconSportsFootball {
  props: crate::Props,
}

impl yew::Component for IconSportsFootball {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M20.31,3.69C19.99,3.36,18.37,3,16.26,3c-3.03,0-7.09,0.75-9.8,3.46C1.87,11.05,2.9,19.52,3.69,20.31 C4.01,20.64,5.63,21,7.74,21c3.03,0,7.09-0.75,9.8-3.46C22.13,12.95,21.1,4.48,20.31,3.69z M7.74,19c-1.14,0-2.02-0.12-2.53-0.23 c-0.18-0.79-0.3-2.21-0.17-3.83l4.01,4.01C8.53,18.99,8.08,19,7.74,19z M16.13,16.13c-1.33,1.33-3.06,2.05-4.66,2.44l-6.04-6.04 c0.42-1.68,1.16-3.37,2.45-4.65c1.32-1.32,3.05-2.04,4.64-2.43l6.05,6.05C18.15,13.17,17.4,14.85,16.13,16.13z M18.96,9.09 l-4.03-4.03C15.45,5.01,15.91,5,16.26,5c1.14,0,2.02,0.12,2.53,0.23C18.97,6.02,19.09,7.45,18.96,9.09z"/><rect height="1.98" transform="matrix(0.7071 -0.7071 0.7071 0.7071 -4.9706 12.0001)" width="7.92" x="8.04" y="11.01"/></g></g></svg>
            </svg>
        }
    }
}

