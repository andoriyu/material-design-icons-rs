
pub struct IconPersonalInjury {
  props: crate::Props,
}

impl yew::Component for IconPersonalInjury {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M12,4c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2s-2-0.9-2-2C10,4.9,10.9,4,12,4z M10.94,20H9.75 C9.34,20,9,19.66,9,19.25c0-0.41,0.34-0.75,0.75-0.75h1.89L10.94,20z M18,20h-4.85l2.94-6.27c0.54,0.2,1.01,0.41,1.4,0.61 C17.8,14.5,18,14.84,18,15.22V20z M9.75,22C8.23,22,7,20.77,7,19.25c0-1.52,1.23-2.75,2.75-2.75h2.83l1.55-3.3 C13.47,13.07,12.76,13,12,13c-2.37,0-4.29,0.73-5.48,1.34C6.2,14.5,6,14.84,6,15.22V22H9.75z" opacity=".3"/><path d="M12,10c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4S8,3.79,8,6C8,8.21,9.79,10,12,10z M12,4c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2 s-2-0.9-2-2C10,4.9,10.9,4,12,4z M18.39,12.56C16.71,11.7,14.53,11,12,11c-2.53,0-4.71,0.7-6.39,1.56C4.61,13.07,4,14.1,4,15.22V22 h2v-6.78c0-0.38,0.2-0.72,0.52-0.88C7.71,13.73,9.63,13,12,13c0.76,0,1.47,0.07,2.13,0.2l-1.55,3.3H9.75C8.23,16.5,7,17.73,7,19.25 C7,20.77,8.23,22,9.75,22h2.18H18c1.1,0,2-0.9,2-2v-4.78C20,14.1,19.39,13.07,18.39,12.56z M10.94,20H9.75C9.34,20,9,19.66,9,19.25 c0-0.41,0.34-0.75,0.75-0.75h1.89L10.94,20z M18,20h-4.85l2.94-6.27c0.54,0.2,1.01,0.41,1.4,0.61C17.8,14.5,18,14.84,18,15.22V20z"/></svg>
            </svg>
        }
    }
}

